//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2663/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663(t28: f64, t5145: f64, t591: f64, t1081: f64, t11122: f64, t12001: f64, t12072: f64, t15952: f64, t15955: f64, t16: f64, t1649: f64, t2: f64, t3672: f64, t39436: f64, t5142: f64, t517: f64, t53832: f64, t53835: f64, t53841: f64, t53844: f64, t584: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t54370 = 32.0_f64 * t5145 * t591;
    let t54372 = piecewise3(t29, 0.0_f64, 40.0_f64 / 81.0_f64 * t39436 * t1649 * t12001 + 16.0_f64 / 9.0_f64 * t12072 * t2 * t53832 - 8.0_f64 / 9.0_f64 * t15952 * t53835 - 8.0_f64 / 3.0_f64 * t3672 * t584 * t1081 + 8.0_f64 * t15955 * t53841 - 8.0_f64 / 3.0_f64 * t15955 * t53844 + 4.0_f64 / 9.0_f64 * t5142 * t11122 + 16.0_f64 * t517 * t16 - t54370);
    t54372
}
