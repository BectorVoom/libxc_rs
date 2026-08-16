//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 635/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk635(t600: f64, t1341: f64, t1347: f64, t3918: f64, t473: f64, t3944: f64, t187: f64, t4114: f64, t1588: f64, t1591: f64, t1590: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4357 = t600 * t600;
    let t4358 = 1.0_f64 / t4357;
    let t4363 = t1341 * t1347;
    let t4366 = t473 * t3918;
    let t4373 = t473 * t3944;
    let t4381 = t187 * t1341;
    let t4399 = 0.38691203703703703703e-3_f64 * t4114;
    let t4409 = t1588 * t1591;
    let t4413 = 1.0_f64 / t1590 / t625;
    (t4357, t4358, t4363, t4366, t4373, t4381, t4399, t4409, t4413)
}
