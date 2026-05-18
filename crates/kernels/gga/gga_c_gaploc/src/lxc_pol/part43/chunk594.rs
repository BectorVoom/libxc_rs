//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 594/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk594<F: Float>(t10470: F, t1537: F, t10241: F, t493: F, t590: F, t1441: F, t10144: F, t1457: F, t1572: F, t8063: F, t895: F, t3377: F, t8155: F) -> (F, F, F, F, F, F) {
    let t10472 = F::new(0.25561950635947166451e1) * t1537 * t10470;
    let t10473 = t493 * t10241;
    let t10474 = t10473 * t590;
    let t10476 = F::new(0.1022478025437886658e1) * t1441 * t10474;
    let t10477 = t1457 * t10144;
    let t10479 = F::new(0.71500979903700853338e0) * t1572 * t10477;
    let t10484 = F::new(0.23833659967900284446e0) * t895 * t8063;
    let t10501 = F::new(0.10725146985555128001e1) * t8155 * t3377;
    (t10472, t10473, t10476, t10479, t10484, t10501)
}
