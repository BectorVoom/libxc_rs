//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1202/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1202(t3139: f64, t332: f64, t2118: f64, t8913: f64, t4395: f64, t814: f64, t2306: f64, t810: f64, t274: f64, t4408: f64, t858: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27729 = t332 * t3139;
    let t28139 = t2118 * t8913;
    let t28652 = t4395 * t814;
    let t28657 = t2306 * t810;
    let t29260 = t4408 * t274;
    let t29751 = t858 * t892;
    (t27729, t28139, t28652, t28657, t29260, t29751)
}
