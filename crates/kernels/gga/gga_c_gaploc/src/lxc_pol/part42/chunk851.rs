//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 851/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk851<F: Float>(t1991: F, t44707: F, t590: F, t739: F, t1890: F, t1966: F, t44712: F, t43370: F, t43373: F, t43377: F, t43383: F, t43386: F) -> (F, F, F, F, F, F, F) {
    let t45170 = F::new(0.20449560508757733161e1) * t1991 * t739 * t44707 * t590;
    let t45174 = F::new(0.25561950635947166451e1) * t1966 * t1890 * t44712 * t590;
    let t45176 = F::new(0.23005755572352449806e1) * t43370;
    let t45177 = F::new(0.23005755572352449806e1) * t43373;
    let t45178 = F::new(0.23005755572352449806e1) * t43377;
    let t45179 = F::new(0.51123901271894332902e0) * t43383;
    let t45180 = F::new(0.15337170381568299871e1) * t43386;
    (t45170, t45174, t45176, t45177, t45178, t45179, t45180)
}
