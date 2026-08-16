//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1823/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1823(t1184: f64, t52: f64, t460: f64, t24682: f64, t3548: f64, t7310: f64, t3469: f64, t7320: f64, t2134: f64, t24650: f64, t24655: f64, t24659: f64, t24664: f64, t24670: f64, t24675: f64, t24677: f64, t24681: f64, t3552: f64, t3557: f64, t3562: f64, t3587: f64, t488: f64, t7316: f64, t7321: f64, t7326: f64, t7331: f64, t7345: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24683 = t52 * t1184;
    let t24684 = t24683 * t460;
    let t24685 = t24682 * t24684;
    let t24690 = t7310 * t3548;
    let t24698 = t3469 * t460;
    let t24699 = t24698 * t7320;
    let t24702 = -0.20186378047070195428e-3_f64 * t24650 * t7331 + 0.10093189023535097714e-3_f64 * t7326 * t24655 + 0.20186378047070195428e-3_f64 * t24659 * t24664 - 0.10093189023535097714e-3_f64 * t24659 * t24670 + t7310 * t3562 / 216.0_f64 + t24675 / 1152.0_f64 + t24677 * t488 / 1536.0_f64 - t24681 - 0.20186378047070195428e-3_f64 * t24685 * t7331 + 5.0_f64 / 6912.0_f64 * t7345 * t3587 - t24690 / 432.0_f64 - t7310 * t3552 / 288.0_f64 - t7310 * t3557 / 144.0_f64 + 0.20186378047070195428e-3_f64 * t7316 * t7321 - 0.10093189023535097714e-3_f64 * t2134 * t24699;
    (t24683, t24684, t24685, t24690, t24698, t24699, t24702)
}
