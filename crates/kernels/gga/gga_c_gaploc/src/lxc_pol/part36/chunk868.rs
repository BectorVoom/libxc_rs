//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 868/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk868<F: Float>(t40395: F, t41839: F, t6710: F, t6711: F, t204: F, t41878: F, t587: F, t2890: F, t9267: F, t9278: F, t40374: F, t40380: F, t40397: F, t40400: F, t42144: F, t42146: F, t42151: F, t42154: F, t42157: F, t42159: F, t42161: F, t42163: F, t42166: F, t42168: F, t42170: F, t42172: F) -> F {
    let t42173 = F::new(0.29792074959875355558e-1) * t40395;
    let t42176 = t6710 * t6711 * t41839;
    let t42180 = t587 * t204 * t41878;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = F::new(0.19171462976960374838e1) * t42183;
    let t42185 = -t42144 - F::new(0.51123901271894332901e0) * t42146 - t42151 + t42154 + t42157 - t42159 - t42161 - F::new(0.12423108009070322895e3) * t42163 + F::new(0.55213813373645879536e2) * t42166 - t42168 - F::new(0.38342925953920749676e0) * t40374 - t42170 + F::new(0.51123901271894332901e0) * t40380 + t42172 + t42173 + F::new(0.38342925953920749676e0) * t40397 - F::new(0.23005755572352449806e2) * t42176 - F::new(0.76685851907841499352e0) * t40400 - F::new(0.18404604457881959845e2) * t42180 + t42184;
    t42185
}
