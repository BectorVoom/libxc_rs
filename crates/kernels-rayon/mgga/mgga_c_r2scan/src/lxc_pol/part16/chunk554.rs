//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 554/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk554(t108: f64, t3039: f64, t3040: f64, t1542: f64, t2892: f64, t3016: f64, t490: f64, t109: f64, t111: f64, t915: f64, t917: f64) -> (f64, f64, f64, f64) {
    let t3042 = (t3039 + t3040) * t108;
    let t3046 = t1542 * t2892;
    let t3049 = t490 * t3016;
    let t3052 = -12.0_f64 * t109 * t3046 + 3.0_f64 * t109 * t3049 - t3042 * t111 + 6.0_f64 * t915 * t917;
    (t3042, t3046, t3049, t3052)
}
