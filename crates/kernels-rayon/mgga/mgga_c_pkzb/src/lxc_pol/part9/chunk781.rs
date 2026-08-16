//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 781/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk781(t5520: f64, t5522: f64, t5525: f64, t5539: f64, t665: f64, t5519: f64, t210: f64, t5512: f64, t1873: f64, t667: f64, t1867: f64, t672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5540 = -t5520 + 4.0_f64 / 3.0_f64 * t5522 - t5525 + t5539;
    let t5541 = t665 * t5540;
    let t5543 = 0.93932222222222222223e0_f64 * t5519;
    let t5547 = 1.0_f64/pow_3_2(t210);
    let t5548 = t5547 * t5512;
    let t5550 = t1873 * t667;
    let t5551 = t5550 * t1867;
    let t5553 = t672 * t5540;
    (t5540, t5541, t5543, t5547, t5548, t5551, t5553)
}
