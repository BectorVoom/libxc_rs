//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 944/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk944(t20233: f64, t8392: f64, t20230: f64, t20240: f64, t20292: f64, t1882: f64, t20307: f64, t103: f64, t20098: f64, t20409: f64, t20288: f64, t1526: f64, t4422: f64, t7705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t75586 = t8392 * t20233;
    let t75588 = t8392 * t20230;
    let t75590 = t8392 * t20240;
    let t75624 = t8392 * t20292;
    let t75642 = t1882 * t20307;
    let t75678 = t103 * t20098;
    let t75766 = t1882 * t20409;
    let t75845 = t8392 * t20288;
    let t75878 = t1526 * t7705 * t4422;
    (t75586, t75588, t75590, t75624, t75642, t75678, t75766, t75845, t75878)
}
