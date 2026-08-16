//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta84 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk491;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk492;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta84(t241: f64, t2690: f64, t244: f64, t248: f64, t238: f64, t835: f64, t841: f64, t812: f64, t1891: f64, t67: f64, t257: f64, t856: f64, t68: f64, t252: f64, t2627: f64, t261: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2691, t2693, t2695, t2696, t2697) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk491(t241, t2690, t244, t248, t238, t835, t841, t812);
        let (t2701, t2718, t2728, t2751, t2752) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk492(t1891, t241, t67, t257, t856, t68, t252, t2627, t261);
    (t2691, t2693, t2695, t2696, t2697, t2701, t2718, t2728, t2751, t2752)
}
