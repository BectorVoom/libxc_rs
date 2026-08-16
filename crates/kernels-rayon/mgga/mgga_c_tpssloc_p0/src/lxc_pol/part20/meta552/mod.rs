//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2097;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2098;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta552(t41362: f64, t831: f64, t2686: f64, t9671: f64, t2681: f64, t2628: f64, t2690: f64, t812: f64, t2635: f64, t9674: f64, t2697: f64, t9618: f64, t40904: f64, t816: f64, t835: f64, t9972: f64, t9978: f64, t9667: f64, t9983: f64, t2617: f64, t9666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41363, t41365, t41373, t41386, t41395, t41397) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2097(t41362, t831, t2686, t9671, t2681, t2628, t2690, t812, t2635, t9674, t2697, t9618);
        let (t41399, t41404, t41415, t41417, t41424) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2098(t40904, t816, t2681, t9674, t812, t835, t9972, t9978, t9667, t9983, t2617, t9666);
    (t41363, t41365, t41373, t41386, t41395, t41397, t41399, t41404, t41415, t41417, t41424)
}
