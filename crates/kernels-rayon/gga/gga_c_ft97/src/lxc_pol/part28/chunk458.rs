//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 458/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk458(t79: f64, t355: f64, t7205: f64, t7204: f64, t1291: f64, t1295: f64, t1303: f64, t5587: f64, t7173: f64, t7178: f64, t7181: f64, t7183: f64, t7191: f64, t7196: f64, t7202: f64) -> (f64, f64, f64) {
    let t80 = 0.1e-59_f64 < t79;
    let t7206 = t7205 * t355;
    let t7207 = t7204 * t7206;
    let t7211 = piecewise3(t80, 2.0_f64 * t7173 - 0.88910709717637694816e-2_f64 * t1295 * t1291 - 0.76612330055555555556e-1_f64 * t7178 * t1303 + 0.22227677429409423704e-2_f64 * t7181 * t7183 + 0.19762785756235085044e-4_f64 * t79 * t7191 + 0.34058283191806748844e-3_f64 * t5587 * t7196 - 0.22227677429409423704e-2_f64 * t79 * t7183 + 0.58694491165413811142e-2_f64 * t7202 * t7207, 0.0_f64);
    (t7206, t7207, t7211)
}
