//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 889/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk889<F: Float>(t43373: F, t43377: F, t43383: F, t43386: F, t36700: F, t44777: F, t549: F, t44712: F, t739: F, t1991: F, t590: F, t43389: F) -> (F, F, F, F, F, F, F) {
    let t45177 = F::new(0.23005755572352449806e1) * t43373;
    let t45178 = F::new(0.23005755572352449806e1) * t43377;
    let t45179 = F::new(0.51123901271894332902e0) * t43383;
    let t45180 = F::new(0.15337170381568299871e1) * t43386;
    let t45183 = F::new(0.47667319935800568892e0) * t36700 * t549 * t44777;
    let t45184 = t739 * t44712;
    let t45187 = F::new(0.1022478025437886658e1) * t1991 * t45184 * t590;
    let t45188 = F::new(0.14570311862489884877e2) * t43389;
    (t45177, t45178, t45179, t45180, t45183, t45187, t45188)
}
