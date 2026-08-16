//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 710/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk710(t1909: f64, t20430: f64, t16076: f64, t925: f64, t16228: f64, t8217: f64, t11939: f64, t16336: f64, t16337: f64, t16338: f64, t16342: f64, t16343: f64, t16346: f64, t20101: f64, t20116: f64, t20136: f64, t20151: f64, t20159: f64, t20316: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20431 = t1909 * t20430;
    let t20434 = t16076 * t925;
    let t20435 = t1909 * t20434;
    let t20438 = t16228 * t925;
    let t20439 = t8217 * t20438;
    let t20448 = t16336 - t16337 + t16338 - t20101 / 3.0_f64 - 2.0_f64 * t20116 + t16342 - t16343 - t16346 - t20316 / 4.0_f64 + 4.0_f64 / 9.0_f64 * t20136 - 2.0_f64 / 3.0_f64 * t20151 - t20159 / 9.0_f64 - t11939;
    (t20431, t20434, t20435, t20438, t20439, t20448)
}
