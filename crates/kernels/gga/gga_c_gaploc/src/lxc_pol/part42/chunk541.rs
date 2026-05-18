//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 541/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk541<F: Float>(t10466: F, t2487: F, t10241: F, t1339: F, t590: F, t1537: F, t493: F, t1441: F, t10144: F, t1457: F, t1572: F, t8063: F, t895: F) -> (F, F, F, F, F, F, F, F) {
    let t10467 = t2487 * t10466;
    let t10468 = F::new(0.25561950635947166451e0) * t10467;
    let t10469 = t1339 * t10241;
    let t10470 = t10469 * t590;
    let t10472 = F::new(0.25561950635947166451e1) * t1537 * t10470;
    let t10473 = t493 * t10241;
    let t10474 = t10473 * t590;
    let t10476 = F::new(0.1022478025437886658e1) * t1441 * t10474;
    let t10477 = t1457 * t10144;
    let t10479 = F::new(0.71500979903700853338e0) * t1572 * t10477;
    let t10484 = F::new(0.23833659967900284446e0) * t895 * t8063;
    (t10467, t10468, t10469, t10472, t10473, t10476, t10479, t10484)
}
