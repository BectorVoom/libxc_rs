//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2627/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2627(t11665: f64, t11668: f64, t11678: f64, t11731: f64, t11770: f64, t14735: f64, t15438: f64, t15708: f64, t15750: f64, t3577: f64, t4723: f64, t52911: f64, t53366: f64, t53453: f64, t53456: f64, t53468: f64, t53470: f64, t53472: f64, t53476: f64) -> f64 {
    let t53478 = 5.0_f64 / 2304.0_f64 * t11678 * t11668 * t4723 * t53366 - t53453 - t15438 * t11770 / 1024.0_f64 - t53456 / 81.0_f64 + 5.0_f64 / 2304.0_f64 * t11665 * t15750 + 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t14735 * t15708 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t4723 * t52911 - t53468 / 2304.0_f64 - t53470 / 1152.0_f64 - t53472 * t11731 / 512.0_f64 - t53476 / 576.0_f64;
    t53478
}
