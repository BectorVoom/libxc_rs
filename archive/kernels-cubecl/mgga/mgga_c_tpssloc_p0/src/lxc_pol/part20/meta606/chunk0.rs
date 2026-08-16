//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2189/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2189<F: Float>(t11687: F, t11697: F, t3577: F, t11877: F, t3576: F, t11647: F, t1203: F, t11859: F, t1222: F, t11797: F, t3490: F, t11172: F, t1227: F, t248: F, t3521: F) -> (F, F, F, F, F, F) {
    let t44994 = t3577 * t11697 * t11687;
    let t44996 = t11877 * t3576;
    let t45002 = t1203 * t11647;
    let t45007 = t11859 * t1222;
    let t45009 = t3490 * t11797;
    let t45013 = t1227 * t248 * t3521 * t11172;
    (t44994, t44996, t45002, t45007, t45009, t45013)
}
