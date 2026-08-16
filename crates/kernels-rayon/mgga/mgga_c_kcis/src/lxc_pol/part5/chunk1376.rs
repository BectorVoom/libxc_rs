//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1376/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1376(t1546: f64, t22652: f64, t4281: f64, t7305: f64, t22471: f64, t22632: f64, t22634: f64, t22638: f64, t22641: f64, t22643: f64, t22645: f64, t22647: f64, t22650: f64) -> (f64, f64, f64) {
    let t22653 = t1546 * t22652;
    let t22655 = t4281 * t7305;
    let t22657 = -t22471 / 576.0_f64 + t22632 / 16.0_f64 + t22634 / 8.0_f64 - t22638 / 256.0_f64 + t22641 / 192.0_f64 + t22643 / 24.0_f64 - t22645 / 8.0_f64 + t22647 / 3.0_f64 + 3.0_f64 / 128.0_f64 * t22650 - t22653 / 24.0_f64 + t22655 / 256.0_f64;
    (t22653, t22655, t22657)
}
