//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1973/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973(t491: f64, t8034: f64, t7287: f64, t24567: f64, t8014: f64, t225: f64, t8018: f64, t1252: f64, t15797: f64, t2155: f64, t24589: f64, t24891: f64, t27800: f64, t27805: f64, t27808: f64, t27812: f64, t27818: f64, t3487: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t7283: f64, t7296: f64, t7351: f64, t7356: f64, t7392: f64, t7999: f64, t8088: f64) -> (f64, f64, f64, f64, f64) {
    let t27820 = t8034 * t491;
    let t27821 = t27820 * t7287;
    let t27826 = t24567 * t8014;
    let t27830 = t8018 * t225;
    let t27832 = -0.82246703342411321825e-2_f64 * t7283 * t27800 - t3487 * t8088 - t4945 * t7392 + t27805 * t498 - t7351 * t5089 - 0.73108180748810063843e-2_f64 * t27808 - 0.21932454224643019153e-1_f64 * t7999 * t7296 - 0.82246703342411321825e-2_f64 * t7283 * t27812 - t5055 * t7392 - 0.91385225936012579807e-3_f64 * t24891 + 0.27415567780803773942e-2_f64 * t27818 + 0.27415567780803773942e-2_f64 * t24589 * t27821 + 2.0_f64 * t5055 * t7356 - 0.82246703342411321825e-2_f64 * t7283 * t27826 - t15797 * t2155 - t27830 * t1252;
    (t27820, t27821, t27826, t27830, t27832)
}
