//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1495;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta488(t54389: f64, t56185: f64, t54392: f64, t74072: f64, t74074: f64, t74077: f64, t54411: f64, t54412: f64, t20416: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t5126: f64, t5127: f64, t6347: f64, t54428: f64, t193: f64, t3918: f64, t3924: f64, t39490: f64, t39496: f64, t39499: f64, t39502: f64, t39505: f64, t39508: f64, t39518: f64, t39521: f64, t39529: f64, t39539: f64, t39549: f64, t39563: f64, t5122: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494(t54389, t56185, t54392, t74072, t74074, t74077, t54411, t54412, t20416, t39411, t39463, t39468, t39472, t39476, t39483, t5126, t5127);
        let t79921 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1495(t6347);
        let (t79925, t79926) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496(t54428, t193, t20416, t3918, t3924, t39490, t39496, t39499, t39502, t39505, t39508, t39518, t39521, t39529, t39539, t39549, t39563, t5122, t79921);
    (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915, t79921, t79925, t79926)
}
