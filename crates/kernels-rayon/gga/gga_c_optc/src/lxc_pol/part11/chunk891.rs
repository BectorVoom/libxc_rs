//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 891/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk891(t16800: f64, t16815: f64, t837: f64, t16784: f64, t7504: f64, t1343: f64, t13998: f64, t3657: f64, t4815: f64, t10493: f64, t4819: f64, t10188: f64, t10348: f64, t13649: f64, t13651: f64, t13653: f64, t13699: f64, t13701: f64, t16716: f64, t16730: f64, t16732: f64, t16734: f64, t16737: f64, t7656: f64, t7657: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16816 = t16800 + t16815;
    let t16817 = t16816 * t837;
    let t16820 = t16784 * t7504;
    let t16824 = 3.0_f64 * t13998 * t1343;
    let t16826 = 3.0_f64 * t3657 * t4815;
    let t16828 = 0.48245472966453314466e2_f64 * t10493 * t4819;
    let t16841 = 0.5477111111111111111e-1_f64 * t13649 - 0.32862666666666666666e0_f64 * t13651 + 0.16431333333333333333e0_f64 * t13653 - 0.76790625e-1_f64 * t16716 - 0.27385555555555555556e0_f64 * t10348 + 0.3071625e0_f64 * t16730 - 0.28483875e1_f64 * t16732 + 0.46074375e0_f64 * t16734 - 0.39862222222222222223e0_f64 * t10188 - t7656 - t7657 + 0.142419375e1_f64 * t16737 + 0.19931111111111111111e0_f64 * t13699 - 0.59793333333333333333e0_f64 * t13701;
    (t16816, t16817, t16820, t16824, t16826, t16828, t16841)
}
