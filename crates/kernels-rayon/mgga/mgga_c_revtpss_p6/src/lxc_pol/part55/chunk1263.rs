//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1263/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1263(t34298: f64, t98588: f64, t2014: f64, t28926: f64, t8717: f64, t28182: f64, t8698: f64, t34261: f64, t7374: f64, t32392: f64, t7978: f64, t32394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t128869 = 2.0_f64 * t98588 * t34298;
    let t128871 = t2014 * t28926 * t8717;
    let t128874 = t8698 * t28182;
    let t128876 = 2.0_f64 * t34261 * t7374;
    let t128878 = 2.0_f64 * t32392 * t7978;
    let t128880 = 2.0_f64 * t32394 * t7978;
    (t128869, t128871, t128874, t128876, t128878, t128880)
}
