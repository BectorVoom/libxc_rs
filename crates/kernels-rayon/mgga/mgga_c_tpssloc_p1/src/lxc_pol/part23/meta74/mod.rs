//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk439;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk440;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta74(t2281: f64, t40: f64, t632: f64, t73: f64, t52: f64, t636: f64, t76: f64, t107: f64, t106: f64, t655: f64, t94: f64, t102: f64, t177: f64, t738: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2282, t2289, t2291, t2296, t2298, t2327, t2331, t2341, t2349) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk439(t2281, t40, t632, t73, t52, t636, t76, t107, t106, t655, t94, t102);
        let (t2367, t2368) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk440(t177, t738);
        let t2369 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk441(t745);
    (t2282, t2289, t2291, t2296, t2298, t2327, t2331, t2341, t2349, t2367, t2368, t2369)
}
