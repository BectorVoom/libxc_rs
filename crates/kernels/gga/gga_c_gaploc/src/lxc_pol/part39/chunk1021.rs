//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1021/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1021<F: Float>(t1457: F, t1572: F, t46920: F, t42380: F, t42381: F, t42385: F, t42388: F, t42390: F, t42392: F, t48167: F, t48172: F, t48175: F, t48178: F, t48182: F, t188: F, t46965: F) -> (F, F) {
    let t48185 = 0.71500979903700853338e0 * t1572 * t1457 * t46920;
    let t48186 = -0.35750489951850426669e0 * t48167 + 0.42900587942220512003e1 * t48172 - 0.11502877786176224903e2 * t48175 - 0.19171462976960374838e0 * t48178 - t48182 + t48185 - t42380 + t42381 - t42385 + t42388 - t42390 + t42392;
    let t48187 = t188 * t46965;
    (t48186, t48187)
}
