//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 621/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk621<F: Float>(t1333: F, t428: F, t4778: F, t87: F, t40: F, t1319: F, t456: F, t4607: F, t470: F, t472: F, t542: F, t1447: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4830 = t1333 * t428;
    let t4831 = F::new(60.0) * t4830;
    let t4832 = t4778 * t87;
    let t4833 = t40 * t4832;
    let t4835 = t1319 * t4607 * t456;
    let t4836 = t470 * t4835;
    let t4837 = F::new(0.35089340384731224426e1) * t4836;
    let t4838 = t542 * t472;
    let t4839 = t1447 * t4838;
    (t4830, t4831, t4832, t4833, t4835, t4836, t4837, t4838, t4839)
}
