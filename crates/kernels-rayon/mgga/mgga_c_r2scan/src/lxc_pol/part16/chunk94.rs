//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 94/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk94(t120: f64, t125: f64, t135: f64, t254: f64, t279: f64, t57: f64) -> (f64, f64) {
    let t282 = 1.0_f64 + 0.27439371595564631661e-1_f64 * t120 * t125 * t57 * t135 + 0.43341108700271342816e-1_f64 * t254 * t279;
    let t283 = pow_1_4(t282);
    (t282, t283)
}
