//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta626<F: Float>(t6875: F, t8944: F, t1845: F, t3698: F, t3734: F, t12813: F, t89: F, t27240: F, t580: F, t1395: F, t7961: F, t1851: F, t7240: F, t1858: F, t7222: F, t1396: F, t16546: F, t1852: F, t2099: F, t24486: F, t27286: F, t3932: F, t5364: F, t5381: F, t7223: F, t84031: F, t85394: F, t85397: F, t671: F, t7039: F, t2035: F, t2363: F, t2319: F, t7786: F, t2032: F, t24001: F, t26076: F, t7026: F, t7035: F, t7435: F, t84174: F, t84196: F, t84198: F, t84200: F, t84203: F, t84205: F, t84207: F, t84220: F, t90160: F, t90297: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91669, t91687, t91695, t91753, t91830, t91832, t91834) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952::<F>(t6875, t8944, t1845, t3698, t3734, t12813, t89, t27240, t580, t1395, t7961, t1851, t7240);
        let t91846 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953::<F>(t1858, t7222, t1396, t16546, t1852, t2099, t24486, t27286, t3932, t5364, t5381, t7223, t7240, t7961, t84031, t85394, t85397, t91830, t91832, t91834);
        let (t91854, t91857, t91870, t91888) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954::<F>(t671, t7039, t2035, t2363, t2319, t7786, t2032, t24001, t26076, t7026, t7035, t7435, t84174, t84196, t84198, t84200, t84203, t84205, t84207, t84220, t90160, t90297);
    (t91669, t91687, t91695, t91753, t91846, t91854, t91857, t91870, t91888)
}
