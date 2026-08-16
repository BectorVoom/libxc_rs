//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 714/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk714(t2040: f64, t2046: f64, t36938: f64, t14091: f64, t35244: f64, t35228: f64, t3154: f64, t34881: f64, t14051: f64, t14367: f64, t14053: f64, t2145: f64, t27: f64, t3118: f64, t664: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t70021 = t2046 * t36938 * t2040;
    let t70048 = t14091 * t35244;
    let t70050 = t14091 * t35228;
    let t70052 = t34881 * t3154;
    let t70062 = t14051 * t14367;
    let t70063 = t70062 * t14053;
    let t70071 = t2145 * t27 * t3118 * t664;
    (t70021, t70048, t70050, t70052, t70062, t70063, t70071)
}
