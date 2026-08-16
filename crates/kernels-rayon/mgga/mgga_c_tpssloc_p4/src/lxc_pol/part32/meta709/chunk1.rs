//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2215/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2215(t23270: f64, t258: f64, t5527: f64, t776: f64, t87642: f64, t6552: f64, t7479: f64, t87782: f64, t13053: f64, t13065: f64, t13463: f64, t17049: f64, t1911: f64, t25348: f64, t2597: f64, t2718: f64, t28307: f64, t28317: f64, t4273: f64, t7517: f64, t7538: f64, t855: f64, t86844: f64, t86869: f64, t86887: f64, t86896: f64, t92383: f64, t98117: f64, t98122: f64, t98125: f64, t98135: f64, t98148: f64) -> f64 {
    let t98153 = t87642 * t23270 * t258 * t5527 * t776;
    let t98158 = t6552 * t87782 * t7479;
    let t98160 = 4.0_f64 * t25348 * t4273 + 0.76763589786250567037e-1_f64 * t98117 - 0.49348022005446793095e-1_f64 * t98122 + 0.3289868133696452873e-1_f64 * t98125 + t86844 + 2.0_f64 * t2597 * t28317 + 2.0_f64 * t855 * t2718 * t1911 * t17049 + t86869 - t92383 - 0.82246703342411321825e-2_f64 * t98135 - 2.0_f64 * t13463 * t7538 + 4.0_f64 * t2597 * t28307 + t86887 + 4.0_f64 * t13053 * t7517 + 4.0_f64 * t13065 * t7517 + 0.16449340668482264365e-1_f64 * t98148 - 0.19739208802178717238e0_f64 * t98153 - 2.0_f64 * t13053 * t7538 - 0.3289868133696452873e-1_f64 * t98158 + t86896;
    t98160
}
