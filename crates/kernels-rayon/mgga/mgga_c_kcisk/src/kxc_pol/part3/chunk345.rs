//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 345/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk345(t1707: f64, t1709: f64, t1695: f64, t606: f64, t164: f64, t353: f64, t579: f64, t24: f64, t657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1710 = t1707 * t1709;
    let t1712 = 0.29896666666666666667e0_f64 * t1695;
    let t1714 = f64::sqrt(t606);
    let t1715 = t1714 * t1709;
    let t1718 = t353 * t164 * t579;
    let t1719 = 0.16431333333333333333e0_f64 * t1718;
    let t1720 = t24 * t657;
    (t1710, t1712, t1714, t1715, t1718, t1719, t1720)
}
