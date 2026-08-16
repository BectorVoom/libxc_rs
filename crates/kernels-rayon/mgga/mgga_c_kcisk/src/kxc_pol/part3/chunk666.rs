//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 666/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk666(t10552: f64, t1685: f64, t4787: f64, t1640: f64, t4703: f64, t4706: f64, t1663: f64, t4705: f64, t1664: f64, t4742: f64, t1665: f64, t4736: f64) -> (f64, f64, f64, f64, f64) {
    let t10554 = t4787 * t10552 * t1685;
    let t10557 = t1640 * t4703;
    let t10559 = 6.0_f64 * t10557 * t4706;
    let t10560 = t4705 * t1663;
    let t10561 = t10560 * t1664;
    let t10563 = 6.0_f64 * t4742 * t10561;
    let t10564 = t1665 * t4736;
    (t10554, t10559, t10560, t10563, t10564)
}
