//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1380;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1381;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta372<F: Float>(t3637: F, t3639: F, t11153: F, t2244: F, t2250: F, t136: F, t3297: F, t11158: F, t9258: F, t3243: F, t1113: F, t11167: F, t11160: F, t690: F, t11169: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43703, t43706, t43711, t43713, t43715, t43717, t43719, t43721, t43723) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1380::<F>(t3637, t3639, t11153, t2244, t2250, t136, t3297, t11158, t9258, t3243, t1113, t11167);
        let (t43725, t43727) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1381::<F>(t1113, t136, t43723, t11160, t690);
        let t43729 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1382::<F>(t11169, t690);
    (t43703, t43706, t43711, t43713, t43715, t43717, t43719, t43721, t43723, t43725, t43727, t43729)
}
