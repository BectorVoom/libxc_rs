//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 32/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk32(t12: f64, t18: f64, t26: f64, t15: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78 = 0.905775e0_f64 * t12;
    let t79 = 0.1100325e0_f64 * t18;
    let t80 = 0.1241775e0_f64 * t26;
    let t81 = 0.51785e1_f64 * t15 + t78 + t79 + t80;
    let t84 = 1.0_f64 + 0.29608749977793437516e2_f64 / t81;
    let t85 = f64::ln(t84);
    (t78, t79, t80, t81, t84, t85)
}
