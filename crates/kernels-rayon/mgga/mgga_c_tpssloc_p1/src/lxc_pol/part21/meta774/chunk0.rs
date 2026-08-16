//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2680/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680(t25: f64, t54408: f64, t54411: f64, t12061: f64, t15937: f64, t16557: f64, t19547: f64, t19552: f64, t21: f64, t2249: f64, t3664: f64, t3665: f64, t39419: f64, t5134: f64, t5397: f64, t54347: f64, t56226: f64, t584: f64, t606: f64, t6305: f64, t9: f64, t9212: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t56298 = 4.0_f64 * t54408;
    let t56299 = 2.0_f64 * t54411;
    let t56323 = piecewise3(t26, 0.0_f64, 40.0_f64 / 81.0_f64 * t39419 * t6305 * t3665 - 64.0_f64 / 27.0_f64 * t15937 * t56226 - 8.0_f64 / 27.0_f64 * t19547 * t2249 + 32.0_f64 / 9.0_f64 * t3664 * t9 * t21 + 16.0_f64 / 9.0_f64 * t5134 * t584 - 16.0_f64 / 3.0_f64 * t5134 * t9212 - 8.0_f64 / 27.0_f64 * t12061 * t5397 * t3665 + 8.0_f64 / 9.0_f64 * t3664 * t16557 * t606 + 4.0_f64 / 9.0_f64 * t19552 * t2249 + t54347);
    (t56298, t56299, t56323)
}
