//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1662/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1662(t1196: f64, t12547: f64, t3520: f64, t5206: f64, t12581: f64, t3531: f64, t43753: f64, t45187: f64, t45190: f64, t12592: f64, t12378: f64, t300: f64) -> (f64, f64, f64, f64, f64) {
    let t45310 = 0.69263436422725855036e2_f64 * t1196 * t3520 * t12547 * t5206;
    let t45312 = 0.4155806185363551302e3_f64 * t3531 * t12581;
    let t45316 = 0.91082604192152556044e5_f64 * t1196 * t45187 * t43753 * t45190;
    let t45318 = 0.4101607543286562663e4_f64 * t3531 * t12592;
    let t45319 = t300 * t12378;
    (t45310, t45312, t45316, t45318, t45319)
}
