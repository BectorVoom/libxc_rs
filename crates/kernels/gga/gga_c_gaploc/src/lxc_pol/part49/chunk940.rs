//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 940/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk940<F: Float>(t41839: F, t6710: F, t6711: F, t204: F, t41878: F, t587: F, t2890: F, t9267: F, t9278: F, t20671: F, t31047: F, t34814: F) -> (F, F, F, F) {
    let t42176 = t6710 * t6711 * t41839;
    let t42180 = t587 * t204 * t41878;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = F::new(0.19171462976960374838e1) * t42183;
    let t42187 = t31047 * t20671 * t34814;
    (t42176, t42180, t42184, t42187)
}
