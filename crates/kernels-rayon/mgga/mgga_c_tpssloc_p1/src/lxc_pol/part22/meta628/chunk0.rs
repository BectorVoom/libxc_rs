//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2163/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2163(t1788: f64, t9214: f64, t2223: f64, t5168: f64, t5157: f64, t9874: f64, t15908: f64, t9885: f64, t9888: f64, t5154: f64, t9713: f64, t9905: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54314 = t9214 * t1788;
    let t54315 = 144.0_f64 * t54314;
    let t54316 = t2223 * t5168;
    let t54317 = 96.0_f64 * t54316;
    let t54325 = t5157 * t9874;
    let t54380 = t15908 * t9885;
    let t54382 = t15908 * t9888;
    let t54389 = t5154 * t9713;
    let t54392 = t5154 * t9905;
    (t54315, t54317, t54325, t54380, t54382, t54389, t54392)
}
