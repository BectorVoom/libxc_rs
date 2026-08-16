//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 889/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk889(t16784: f64, t2476: f64, t10188: f64, t10348: f64, t13649: f64, t13651: f64, t13653: f64, t13699: f64, t13701: f64, t16716: f64, t16730: f64, t16732: f64, t16734: f64, t16737: f64, t7593: f64, t7594: f64) -> (f64, f64) {
    let t16785 = t16784 * t2476;
    let t16800 = 0.5519e-1_f64 * t13649 - 0.33114e0_f64 * t13651 + 0.16557e0_f64 * t13653 - 0.412621875e-1_f64 * t16716 - 0.27595e0_f64 * t10348 + 0.16504875e0_f64 * t16730 - 0.3883875e1_f64 * t16732 + 0.247573125e0_f64 * t16734 - 0.40256666666666666668e0_f64 * t10188 - t7593 - t7594 + 0.19419375e1_f64 * t16737 + 0.20128333333333333333e0_f64 * t13699 - 0.60385000000000000001e0_f64 * t13701;
    (t16785, t16800)
}
