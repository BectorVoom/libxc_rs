//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 754/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk754<F: Float>(t8018: F, t8023: F, t4826: F, t4837: F, t4840: F, t4843: F, t4846: F, t4849: F, t4854: F, t4856: F, t4858: F, t4861: F, t4864: F) -> (F, F, F) {
    let t12377 = F::new(0.51947267698127589899e2) * t8018;
    let t12378 = F::new(0.35089340384731224426e1) * t8023;
    let t12379 = -t12377 + t12378 + t4826 - t4837 - t4840 - t4843 + t4846 + t4849 + t4854 - t4856 - t4858 - t4861 - t4864;
    (t12377, t12378, t12379)
}
