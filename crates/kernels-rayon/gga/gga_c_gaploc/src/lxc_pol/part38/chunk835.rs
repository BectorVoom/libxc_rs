//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 835/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk835(t11408: f64, t2268: f64, t6320: f64, t6509: f64, t13265: f64, t484: f64, t13296: f64, t599: f64, t475: f64, t3516: f64, t874: f64) -> (f64, f64, f64, f64, f64) {
    let t44375 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t11408 * t6509;
    let t44376 = t484 * t13265;
    let t44377 = 0.47425011059460249332e-2_f64 * t44376;
    let t44381 = t599 * t13296;
    let t44382 = t44381 * t475;
    let t44386 = t3516 * t874;
    (t44375, t44377, t44381, t44382, t44386)
}
