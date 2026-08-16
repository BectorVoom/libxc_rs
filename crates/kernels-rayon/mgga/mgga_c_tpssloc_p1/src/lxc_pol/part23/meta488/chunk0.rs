//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1494/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494(t54389: f64, t56185: f64, t54392: f64, t74072: f64, t74074: f64, t74077: f64, t54411: f64, t54412: f64, t20416: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t39483: f64, t5126: f64, t5127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t79904 = 0.23392894490538584828e1_f64 * t54389;
    let t79905 = 48.0_f64 * t56185;
    let t79906 = 0.14035736694323150897e2_f64 * t54392;
    let t79907 = 16.0_f64 * t74072;
    let t79908 = 16.0_f64 * t74074;
    let t79909 = 0.23392894490538584828e1_f64 * t74077;
    let t79910 = 4.0_f64 * t54411;
    let t79914 = 48.0_f64 * t54412;
    let t79915 = 24.0_f64 * t20416 * t5126 * t5127 + t39411 + t39463 - t39468 - t39472 - t39476 + t39483 - t79904 - t79905 + t79906 - t79907 - t79908 - t79909 + t79910 - t79914;
    (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915)
}
