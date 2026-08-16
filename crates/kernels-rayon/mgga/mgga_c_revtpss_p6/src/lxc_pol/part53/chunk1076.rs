//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1076/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1076(t7901: f64, t8764: f64, t1519: f64, t32825: f64, t33906: f64, t33910: f64, t33914: f64, t33916: f64, t33920: f64, t33977: f64, t34444: f64, t34447: f64, t34449: f64, t34462: f64, t569: f64, t6985: f64, t8158: f64, t8463: f64) -> f64 {
    let t34464 = t8764 * t7901;
    let t34466 = -2.0_f64 * t1519 * t32825 + t34462 * t569 - 2.0_f64 * t6985 * t8158 - 2.0_f64 * t33906 + t33910 + t33914 - t33916 + t33920 + t33977 - 2.0_f64 * t34444 - 2.0_f64 * t34447 - 2.0_f64 * t34449 + 3.0_f64 * t34464 - t8463;
    t34466
}
