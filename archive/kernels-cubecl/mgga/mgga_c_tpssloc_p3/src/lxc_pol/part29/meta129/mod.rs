//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta129<F: Float>(t2862: F, t932: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F, F, F) {
        let (t2863, t2868, t2875, t2880) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk753::<F>(t2862, t932, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
    (t2863, t2868, t2875, t2880)
}
