//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2675/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675(t1388: f64, t5187: f64, t1307: f64, t5356: f64, t54392: f64, t54395: f64, t54398: f64, t54400: f64, t15904: f64, t20077: f64, t20085: f64, t3734: f64, t3918: f64, t39463: f64, t39468: f64, t39472: f64, t5126: f64, t5161: f64) -> (f64, f64, f64, f64, f64) {
    let t56194 = t5187 * t1388;
    let t56198 = t1307 * t5356;
    let t56202 = 0.70178683471615754484e1_f64 * t54392;
    let t56203 = 0.36622894612013090108e-3_f64 * t54395;
    let t56207 = 2.0_f64 * t54398;
    let t56208 = 80.0_f64 * t54400;
    let t56212 = 12.0_f64 * t15904 * t20085 * t3918 - 6.0_f64 * t20077 * t3734 * t5126 - 12.0_f64 * t3918 * t5161 * t56194 - 12.0_f64 * t3918 * t5161 * t56198 + t39463 - t39468 - t39472 + t56202 - t56203 + t56207 + t56208;
    (t56202, t56203, t56207, t56208, t56212)
}
