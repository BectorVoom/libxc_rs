//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta555<F: Float>(t81146: F, t81153: F, t225: F, t24162: F, t81317: F, t24064: F, t81398: F, t2056: F, t40772: F, t24334: F, t2752: F, t193: F, t201: F, t7109: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t84595, t84597, t84655, t84659, t84700, t84705, t84766, t84791, t84797) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1826::<F>(t81146, t81153, t225, t24162, t81317, t24064, t81398, t2056, t40772, t24334, t2752, t193, t201, t7109);
    (t84595, t84597, t84655, t84659, t84700, t84705, t84766, t84791, t84797)
}
