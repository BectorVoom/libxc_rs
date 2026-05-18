//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 888/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk888<F: Float>(t11701: F, t2728: F, t11556: F, t2355: F, t1991: F, t44707: F, t590: F, t739: F, t1890: F, t1966: F, t44712: F, t43370: F) -> (F, F, F, F, F) {
    let t45163 = t11701 * t2728;
    let t45164 = t2355 * t11556;
    let t45170 = F::new(0.20449560508757733161e1) * t1991 * t739 * t44707 * t590;
    let t45174 = F::new(0.25561950635947166451e1) * t1966 * t1890 * t44712 * t590;
    let t45176 = F::new(0.23005755572352449806e1) * t43370;
    (t45163, t45164, t45170, t45174, t45176)
}
