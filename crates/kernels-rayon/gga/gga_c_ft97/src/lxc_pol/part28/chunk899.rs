//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 899/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk899(t147: f64, t34978: f64, t35237: f64, t184: f64, t1080: f64, t21: f64, t33234: f64, t5: f64, t7420: f64, t920: f64, t14: f64, t7194: f64, t72: f64) -> (f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t35238 = t34978 + t35237;
    let t35239 = t35238 * t184;
    let t35247 = piecewise3(t148, 0.0_f64, t5 * t35239 * t21 / 4.0_f64 + t5 * t7420 * t920 / 4.0_f64 + t33234 * t1080 / 4.0_f64);
    let t36363 = t7194 * t14;
    let t36364 = t36363 * t72;
    (t35238, t35239, t35247, t36363, t36364)
}
