//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2058;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta617<F: Float>(t3545: F, t7372: F, t7378: F, t24698: F, t7327: F, t2121: F, t3427: F, t7381: F, t24574: F, t24795: F, t24799: F, t3590: F, t477: F, t7365: F, t85660: F, t1170: F, t24829: F, t131: F, t467: F, t50: F, t82510: F, t10469: F, t461: F, t11721: F, t3032: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85917, t85918, t85920, t85941, t85943, t85945, t85947) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2058::<F>(t3545, t7372, t7378, t24698, t7327, t2121, t3427, t7381, t24574, t24795, t24799, t3590, t477);
        let (t85952, t85955, t85963, t85964, t85966) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2059::<F>(t7365, t85660, t1170, t2121, t24829, t131, t467, t50, t82510, t10469, t461, t11721, t3032);
    (t85917, t85918, t85920, t85941, t85943, t85945, t85947, t85952, t85955, t85963, t85964, t85966)
}
