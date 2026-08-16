//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 549/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk549(t44: f64, t51: f64, t3016: f64, t471: f64, t97: f64, t2491: f64, t1361: f64, t2999: f64, t3002: f64, t48: f64, t1368: f64, t3007: f64, t3010: f64, t53: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t3018 = t97 * t471 * t3016;
    let t3019 = 3.0_f64 * t3018;
    let t3020 = 0.11696447245269292414e1_f64 * t2491;
    let t3026 = piecewise3(t45, 0.0_f64, 4.0_f64 / 9.0_f64 * t1361 * t2999 + 4.0_f64 / 3.0_f64 * t48 * t3002);
    let t3032 = piecewise3(t52, 0.0_f64, 4.0_f64 / 9.0_f64 * t1368 * t3007 + 4.0_f64 / 3.0_f64 * t53 * t3010);
    (t3019, t3020, t3026, t3032)
}
