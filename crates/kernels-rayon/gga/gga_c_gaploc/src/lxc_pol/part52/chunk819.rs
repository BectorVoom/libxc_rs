//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 819/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk819(t2268: f64, t2343: f64, t44470: f64, t11259: f64, t2293: f64, t6320: f64, t13262: f64, t6305: f64, t36178: f64, t874: f64, t13268: f64, t6313: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44473 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t44470;
    let t44474 = t11259 * t2293;
    let t44477 = 0.17073003981405689759e0_f64 * t2268 * t6320 * t44474;
    let t44479 = 0.56910013271352299198e-1_f64 * t6305 * t13262;
    let t44480 = t36178 * t874;
    let t44483 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t44480;
    let t44485 = 0.45528010617081839357e0_f64 * t6313 * t13268;
    (t44473, t44474, t44477, t44479, t44480, t44483, t44485)
}
