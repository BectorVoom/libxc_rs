//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1236/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1236<F: Float>(t237: F, t30377: F, t1084: F, t26283: F, t30242: F, t30245: F, t30248: F, t30252: F, t30255: F, t30259: F, t30261: F, t30263: F, t30265: F, t30268: F, t30270: F, t30273: F, t30275: F, t30277: F, t30362: F, t30364: F, t30366: F, t30369: F) -> (F, F, F) {
    let t30379 = F::new(0.19751673498613801407e-1) * t237 * t30377;
    let t30381 = F::new(3.0) * t26283 * t1084;
    let t30382 = -t30242 - t30245 - t30248 + t30252 + t30255 + t30259 + t30261 - t30263 - t30265 - t30268 - t30270 + t30273 - t30275 - t30277 + t30362 + t30364 + t30366 + t30369 + t30379 + t30381;
    (t30379, t30381, t30382)
}
