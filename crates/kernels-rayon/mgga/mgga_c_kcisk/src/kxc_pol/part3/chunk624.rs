//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 624/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk624(t776: f64, t2028: f64, t791: f64, t1992: f64, t794: f64, t772: f64, t41: f64, t4794: f64, t1758: f64, t1995: f64, t4973: f64, t4977: f64, t525: f64, t642: f64, t773: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t777 = t776 < -0.66725e-1_f64;
    let t5437 = t2028 * t2028;
    let t5438 = t791 * t791;
    let t5439 = 1.0_f64 / t5438;
    let t5440 = t5437 * t5439;
    let t5444 = 1.0_f64 / t1992 / t794;
    let t5445 = t772 * t5444;
    let t5449 = t4794 * t41;
    let t5463 = piecewise3(t777, 0.0_f64, 10.0_f64 / 9.0_f64 * t525 * t5449 * t642 - 20.0_f64 / 27.0_f64 * t525 * t1995 * t1758 + 40.0_f64 / 81.0_f64 * t525 * t773 * t4973 - 10.0_f64 / 27.0_f64 * t525 * t773 * t4977);
    (t5437, t5438, t5439, t5440, t5444, t5445, t5449, t5463)
}
