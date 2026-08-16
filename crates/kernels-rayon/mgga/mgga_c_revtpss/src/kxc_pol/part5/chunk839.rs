//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 839/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk839(t45: f64, t57: f64, t4397: f64, t2375: f64, t5819: f64, t5825: f64, t78: f64, t2382: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, t1522: f64, t4311: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t5927 = 2.0_f64 * t4397;
    let t5933 = piecewise3(t151, 0.0_f64, 4.0_f64 / 9.0_f64 * t2375 * t5819 + 4.0_f64 / 3.0_f64 * t78 * t5825);
    let t5939 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t2382 * t5819 - 4.0_f64 / 3.0_f64 * t81 * t5825);
    let t5940 = t5933 + t5939;
    let t5941 = t5940 * t162;
    let t5943 = 0.19751673498613801407e-1_f64 * t5941 * t187;
    let t5944 = t150 * t5940;
    let t5945 = t5944 * t190;
    let t5947 = 8.0_f64 * t4311 * t1522;
    (t5927, t5940, t5941, t5943, t5944, t5945, t5947)
}
