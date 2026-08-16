//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1327/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1327(t28849: f64, t28874: f64, t28906: f64, t28930: f64, t3418: f64, t847: f64, t20703: f64, t20706: f64, t20904: f64, t24556: f64, t24559: f64, t24562: f64, t284: f64, t28853: f64, t28856: f64, t28859: f64) -> (f64, f64, f64, f64) {
    let t28932 = t28849 + t28874 + t28906 + t28930;
    let t28937 = t3418 * t3418;
    let t28949 = t847 * t3418;
    let t28962 = (t20904 - 0.57685185185185185184e-1_f64 * t20703 + 0.12361111111111111111e-1_f64 * t20706 - 0.57685185185185185187e-1_f64 * t24556 + 0.49444444444444444446e-1_f64 * t24559 - 0.18541666666666666667e-1_f64 * t24562 + 0.12361111111111111111e-1_f64 * t28859 - 0.18541666666666666667e-1_f64 * t28853 + 0.278125e-1_f64 * t28856) * t284;
    (t28932, t28937, t28949, t28962)
}
