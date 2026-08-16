//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1710;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta333(t111: f64, t1395: f64, t5107: f64, t671: f64, t1266: f64, t4072: f64, t1774: f64, t2363: f64, t584: f64, t9212: f64, t9214: f64, t9216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12524, t12545, t12550, t12557, t12560, t12561, t12562, t12563) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1710(t111, t1395, t5107, t671, t1266, t4072, t1774, t2363, t584, t9212, t9214, t9216);
    (t12524, t12545, t12550, t12557, t12560, t12561, t12562, t12563)
}
