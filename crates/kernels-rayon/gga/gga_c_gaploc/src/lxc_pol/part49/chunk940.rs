//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 940/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk940(t41839: f64, t6710: f64, t6711: f64, t204: f64, t41878: f64, t587: f64, t2890: f64, t9267: f64, t9278: f64, t20671: f64, t31047: f64, t34814: f64) -> (f64, f64, f64, f64) {
    let t42176 = t6710 * t6711 * t41839;
    let t42180 = t587 * t204 * t41878;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = 0.19171462976960374838e1_f64 * t42183;
    let t42187 = t31047 * t20671 * t34814;
    (t42176, t42180, t42184, t42187)
}
