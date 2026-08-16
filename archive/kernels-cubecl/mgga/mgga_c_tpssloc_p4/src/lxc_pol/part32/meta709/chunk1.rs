//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2215/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2215<F: Float>(t23270: F, t258: F, t5527: F, t776: F, t87642: F, t6552: F, t7479: F, t87782: F, t13053: F, t13065: F, t13463: F, t17049: F, t1911: F, t25348: F, t2597: F, t2718: F, t28307: F, t28317: F, t4273: F, t7517: F, t7538: F, t855: F, t86844: F, t86869: F, t86887: F, t86896: F, t92383: F, t98117: F, t98122: F, t98125: F, t98135: F, t98148: F) -> F {
    let t98153 = t87642 * t23270 * t258 * t5527 * t776;
    let t98158 = t6552 * t87782 * t7479;
    let t98160 = F::cast_from(4.0_f64) * t25348 * t4273 + F::cast_from(0.76763589786250567037e-1_f64) * t98117 - F::cast_from(0.49348022005446793095e-1_f64) * t98122 + F::cast_from(0.3289868133696452873e-1_f64) * t98125 + t86844 + F::cast_from(2.0_f64) * t2597 * t28317 + F::cast_from(2.0_f64) * t855 * t2718 * t1911 * t17049 + t86869 - t92383 - F::cast_from(0.82246703342411321825e-2_f64) * t98135 - F::cast_from(2.0_f64) * t13463 * t7538 + F::cast_from(4.0_f64) * t2597 * t28307 + t86887 + F::cast_from(4.0_f64) * t13053 * t7517 + F::cast_from(4.0_f64) * t13065 * t7517 + F::cast_from(0.16449340668482264365e-1_f64) * t98148 - F::cast_from(0.19739208802178717238e0_f64) * t98153 - F::cast_from(2.0_f64) * t13053 * t7538 - F::cast_from(0.3289868133696452873e-1_f64) * t98158 + t86896;
    t98160
}
