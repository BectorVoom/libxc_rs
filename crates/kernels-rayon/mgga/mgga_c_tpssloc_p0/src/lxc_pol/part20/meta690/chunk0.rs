//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2618/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2618(t11718: f64, t52835: f64, t11797: f64, t5024: f64, t11147: f64, t15394: f64, t11665: f64, t11724: f64, t11774: f64, t15455: f64, t15459: f64, t15463: f64, t3447: f64, t3490: f64, t45108: f64, t45112: f64, t45126: f64, t45148: f64, t45971: f64, t5005: f64) -> f64 {
    let t53238 = t52835 * t11718;
    let t53246 = t5024 * t11797;
    let t53249 = t15394 * t11147;
    let t53258 = t53238 * t11724 / 512.0_f64 - t45108 / 1152.0_f64 - t45112 - t11665 * t15459 / 1536.0_f64 - t11665 * t15463 / 768.0_f64 + t53246 / 432.0_f64 + 5.0_f64 / 6912.0_f64 * t45126 - 7.0_f64 / 216.0_f64 * t3447 * t53249 * t45971 - t45148 / 1536.0_f64 + 5.0_f64 / 4608.0_f64 * t5005 * t11774 - 5.0_f64 / 1728.0_f64 * t3490 * t15455;
    t53258
}
