//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1197/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1197(t113131: f64, t118436: f64, t118439: f64, t118440: f64, t118455: f64, t118465: f64, t118467: f64, t118949: f64, t118954: f64, t1877: f64, t22959: f64, t25: f64, t25021: f64, t25024: f64, t25028: f64, t2522: f64, t25366: f64, t25372: f64, t25375: f64, t25377: f64, t25381: f64, t25385: f64, t25392: f64, t30757: f64, t30770: f64, t32886: f64, t606: f64, t6542: f64, t8366: f64, t8370: f64) -> f64 {
    let t118964 = 3.0_f64 / 2.0_f64 * t2522 * t32886 * t6542 - 3.0_f64 / 2.0_f64 * t2522 * t8370 * t25024 + t118436 * t25375 - 3.0_f64 * t118439 * t118440 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25028 - t1877 * t30757 * t25381 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25385 + t1877 * t30770 * t25392 - 3.0_f64 * t22959 * t118455 + 3.0_f64 / 2.0_f64 * t2522 * t8366 * t25024 - t1877 * t30757 * t25392 / 2.0_f64 + t118465 - 3.0_f64 * t22959 * t118467 - 3.0_f64 / 2.0_f64 * t113131 * t25021 + t1877 * t118949 * t25 / 2.0_f64 + 2.0_f64 * t25372 * t118954 + t1877 * t32886 * t606 / 2.0_f64 + t1877 * t30770 * t25377 - 3.0_f64 / 2.0_f64 * t113131 * t25366;
    t118964
}
