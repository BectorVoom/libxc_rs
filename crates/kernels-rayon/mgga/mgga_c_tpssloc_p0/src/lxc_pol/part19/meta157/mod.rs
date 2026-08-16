//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk771;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta157(t9287: f64, t9288: f64, t2267: f64, t607: f64, t2250: f64, t43: f64, t9258: f64, t53: f64, t54: f64, t2274: f64, t55: f64, t2585: f64, t2262: f64, t2268: f64, t2271: f64, t39: f64, t44: f64, t51: f64, t615: f64, t618: f64, t9277: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t9289, t9292, t9293, t9296, t9300, t9301, t9304, t9305, t9308, t9311) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk771(t9287, t9288, t2267, t607, t2250, t43, t9258, t53, t54, t2274, t55, t2585);
        let t9312 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk772(t2262, t2268, t2271, t39, t44, t51, t615, t618, t9277, t9289, t9293, t9296, t9301, t9305, t9308, t9311);
    (t9289, t9292, t9293, t9296, t9300, t9304, t9312)
}
