//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 810/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk810(t1117: f64, t1134: f64, t1145: f64, t1158: f64, t1172: f64, t1539: f64, t2821: f64, t2829: f64, t2834: f64, t2838: f64, t2847: f64, t2862: f64, t2868: f64, t2875: f64, t2881: f64, t3760: f64, t4491: f64, t4494: f64, t4502: f64, t4505: f64, t4513: f64, t4521: f64, t4525: f64, t4530: f64) -> f64 {
    let t4534 = -16.0_f64 / 9.0_f64 * t2821 * t4491 + 16.0_f64 / 9.0_f64 * t2829 * t4494 + 16.0_f64 / 3.0_f64 * t2838 * t4494 - 16.0_f64 / 3.0_f64 * t2834 * t4491 - 32.0_f64 / 81.0_f64 * t2862 * t4502 - 16.0_f64 / 27.0_f64 * t1172 * t4505 - 32.0_f64 / 81.0_f64 * t2847 * t4502 - 16.0_f64 / 27.0_f64 * t1158 * t4505 + 44.0_f64 / 27.0_f64 * t1172 * t4513 + 44.0_f64 / 27.0_f64 * t1158 * t4513 - 72.0_f64 * t1134 * t3760 * t1539 - 8.0_f64 * t1117 * t4521 + 21.0_f64 * t2875 * t4525 + 3.0_f64 * t2881 * t4525 + 15.0_f64 * t2868 * t1145 * t4530;
    t4534
}
