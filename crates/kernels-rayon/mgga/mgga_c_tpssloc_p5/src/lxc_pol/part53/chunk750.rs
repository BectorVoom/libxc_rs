//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 750/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk750(t22893: f64, t6969: f64, t22892: f64, t3787: f64, t6604: f64, t22751: f64, t6892: f64, t6883: f64, t6908: f64, t22674: f64, t6891: f64, t1988: f64, t22716: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22894 = t22893 * t6969;
    let t22895 = t22892 * t22894;
    let t22896 = 0.16449340668482264365e-1_f64 * t22895;
    let t22897 = t6604 * t3787;
    let t22907 = t22751 * t6892;
    let t22908 = 0.76763589786250567036e-1_f64 * t22907;
    let t22909 = t6883 * t6908;
    let t22910 = 0.38381794893125283518e-1_f64 * t22909;
    let t22920 = t22674 * t6891;
    let t22921 = t22892 * t22920;
    let t22922 = 0.16449340668482264365e-1_f64 * t22921;
    let t22923 = t22716 * t1988;
    (t22896, t22897, t22908, t22910, t22922, t22923)
}
