//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 696/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk696(t20224: f64, t3187: f64, t1909: f64, t3194: f64, t3193: f64, t11902: f64, t4607: f64, t11906: f64, t4612: f64, t16034: f64, t925: f64, t110: f64, t1866: f64, t20027: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20225 = t3187 * t20224;
    let t20226 = t1909 * t20225;
    let t20229 = t3194 * t20224;
    let t20230 = t3193 * t20229;
    let t20233 = t11902 * t4607;
    let t20236 = t11906 * t4612;
    let t20239 = t16034 * t925;
    let t20240 = t1909 * t20239;
    let t20244 = t1866 * t110 * t20027;
    (t20225, t20226, t20229, t20230, t20233, t20236, t20239, t20240, t20244)
}
