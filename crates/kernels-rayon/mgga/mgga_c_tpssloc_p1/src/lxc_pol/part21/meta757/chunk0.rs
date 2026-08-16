//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2631/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2631(t15977: f64, t588: f64, t5157: f64, t9874: f64, t5137: f64, t591: f64, t5145: f64, t15908: f64, t9885: f64, t9888: f64, t15968: f64, t172: f64, t763: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54323 = t588 * t15977;
    let t54325 = t5157 * t9874;
    let t54347 = 32.0_f64 * t5137 * t591;
    let t54370 = 32.0_f64 * t5145 * t591;
    let t54380 = t15908 * t9885;
    let t54382 = t15908 * t9888;
    let t54387 = t15968 * t172 * t763;
    (t54323, t54325, t54347, t54370, t54380, t54382, t54387)
}
