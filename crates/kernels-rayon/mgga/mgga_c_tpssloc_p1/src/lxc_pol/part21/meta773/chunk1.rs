//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2676/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676(t25: f64, t54402: f64, t2: f64, t584: f64, t606: f64, t11987: f64, t15989: f64, t16557: f64, t19606: f64, t19611: f64, t21: f64, t2249: f64, t3665: f64, t3704: f64, t39861: f64, t5170: f64, t53825: f64, t5397: f64, t6305: f64, t9: f64, t9212: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t56219 = 32.0_f64 * t54402;
    let t56226 = t606 * t2 * t584;
    let t56247 = piecewise3(t26, 0.0_f64, -56.0_f64 / 81.0_f64 * t39861 * t6305 * t3665 + 64.0_f64 / 27.0_f64 * t15989 * t56226 + 8.0_f64 / 27.0_f64 * t19606 * t2249 - 16.0_f64 / 9.0_f64 * t3704 * t9 * t21 - 8.0_f64 / 9.0_f64 * t5170 * t584 + 8.0_f64 / 3.0_f64 * t5170 * t9212 + 8.0_f64 / 27.0_f64 * t11987 * t5397 * t3665 - 4.0_f64 / 9.0_f64 * t3704 * t16557 * t606 - 2.0_f64 / 9.0_f64 * t19611 * t2249 + t53825);
    (t56219, t56226, t56247)
}
