//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2169/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2169(t40406: f64, t5202: f64, t12199: f64, t16111: f64, t1804: f64, t40005: f64, t2585: f64, t3732: f64, t46853: f64, t5308: f64, t16118: f64, t9577: f64) -> (f64, f64, f64, f64, f64) {
    let t54633 = t40406 * t5202;
    let t54637 = t12199 * t16111;
    let t54638 = 0.15833333333333333333e-1_f64 * t54637;
    let t54639 = t40005 * t1804;
    let t54643 = t2585 * t3732 * t46853 * t5308;
    let t54644 = 0.14999999999999999999e-1_f64 * t54643;
    let t54663 = t9577 * t16118;
    (t54633, t54638, t54639, t54644, t54663)
}
