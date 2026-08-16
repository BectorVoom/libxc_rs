//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta443<F: Float>(t1860: F, t23993: F, t6509: F, t7031: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22645: F, t225: F, t7192: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1590::<F>(t1860, t23993, t6509, t7031, t22819, t22825, t22858, t22863, t22867, t22645, t225, t7192);
    (t23995, t23998, t23999, t24049, t24050, t24058, t24060, t24061, t24071, t24082)
}
