//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta626(t6875: f64, t8944: f64, t1845: f64, t3698: f64, t3734: f64, t12813: f64, t89: f64, t27240: f64, t580: f64, t1395: f64, t7961: f64, t1851: f64, t7240: f64, t1858: f64, t7222: f64, t1396: f64, t16546: f64, t1852: f64, t2099: f64, t24486: f64, t27286: f64, t3932: f64, t5364: f64, t5381: f64, t7223: f64, t84031: f64, t85394: f64, t85397: f64, t671: f64, t7039: f64, t2035: f64, t2363: f64, t2319: f64, t7786: f64, t2032: f64, t24001: f64, t26076: f64, t7026: f64, t7035: f64, t7435: f64, t84174: f64, t84196: f64, t84198: f64, t84200: f64, t84203: f64, t84205: f64, t84207: f64, t84220: f64, t90160: f64, t90297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91669, t91687, t91695, t91753, t91830, t91832, t91834) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1952(t6875, t8944, t1845, t3698, t3734, t12813, t89, t27240, t580, t1395, t7961, t1851, t7240);
        let t91846 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1953(t1858, t7222, t1396, t16546, t1852, t2099, t24486, t27286, t3932, t5364, t5381, t7223, t7240, t7961, t84031, t85394, t85397, t91830, t91832, t91834);
        let (t91854, t91857, t91870, t91888) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954(t671, t7039, t2035, t2363, t2319, t7786, t2032, t24001, t26076, t7026, t7035, t7435, t84174, t84196, t84198, t84200, t84203, t84205, t84207, t84220, t90160, t90297);
    (t91669, t91687, t91695, t91753, t91846, t91854, t91857, t91870, t91888)
}
