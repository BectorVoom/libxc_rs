//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta644<F: Float>(t10970: F, t820: F, t1041: F, t10868: F, t248: F, t2780: F, t10277: F, t976: F, t11046: F, t42387: F, t10457: F, t10936: F, t3180: F, t10401: F, t10935: F, t3186: F, t3200: F, t11051: F, t3069: F, t3036: F, t3087: F, t3033: F, t3128: F, t10402: F, t11034: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42397, t42432, t42444, t42483, t42488, t42496) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2435::<F>(t10970, t820, t1041, t10868, t248, t2780, t10277, t976, t11046, t42387, t10457, t10936, t3180);
        let (t42505, t42508, t42511, t42520, t42522, t42541) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2436::<F>(t10401, t10935, t3186, t3200, t11051, t3069, t3036, t3087, t3033, t3128, t10402, t11034);
    (t42397, t42432, t42444, t42483, t42488, t42496, t42505, t42508, t42511, t42520, t42522, t42541)
}
