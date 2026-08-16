//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta130 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk717;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk718;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk719;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk720;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta130<F: Float>(t2904: F, t315: F, t950: F, t951: F, t2764: F, t2822: F, t2766: F, t2773: F, t2778: F, t2782: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F, t941: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2905, t2906) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk717::<F>(t2904, t315, t950);
        let (t2907, t2912, t2919, t2924) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk718::<F>(t2906, t951, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
        let (t2925, t2928, t2929) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk719::<F>(t2924, t951, t941);
        let (t2930, t2931, t2932) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk720::<F>(t2929, t315, t323);
    (t2905, t2906, t2907, t2912, t2919, t2924, t2925, t2928, t2929, t2930, t2931, t2932)
}
