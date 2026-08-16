//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2440;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta646<F: Float>(t3147: F, t698: F, t973: F, t10632: F, t2924: F, t10510: F, t3114: F, t10508: F, t248: F, t3039: F, t3041: F, t3020: F, t3030: F, t3032: F, t3038: F, t1043: F, t204: F, t1041: F, t884: F, t10189: F, t3014: F, t10337: F, t964: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42613, t42671, t42721, t42735, t42741) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2439::<F>(t3147, t698, t973, t10632, t2924, t10510, t3114, t10508, t248, t3039, t3041, t3020, t3030);
        let (t42742, t42743, t42749, t42752, t42771, t42811) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2440::<F>(t3032, t42741, t3038, t1043, t204, t1041, t248, t884, t10189, t3014, t10337, t964);
    (t42613, t42671, t42721, t42735, t42741, t42742, t42743, t42749, t42752, t42771, t42811)
}
