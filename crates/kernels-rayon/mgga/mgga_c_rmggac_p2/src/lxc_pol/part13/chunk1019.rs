//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1019/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1019(t8347: f64, t8353: f64, t8359: f64, t8363: f64, t8366: f64, t8369: f64, t8405: f64, t8408: f64, t8411: f64, t8414: f64, t7270: f64, t7280: f64, t7289: f64, t8034: f64, t8035: f64, t8037: f64, t8039: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42373 = 0.1440846329149835838e-2_f64 * t8347;
    let t42374 = 0.1440846329149835838e-2_f64 * t8353;
    let t42375 = 0.1440846329149835838e-2_f64 * t8359;
    let t42376 = 0.1440846329149835838e-2_f64 * t8363;
    let t42377 = 0.5454932330849068346e-1_f64 * t8366;
    let t42378 = 0.13637330827122670865e-1_f64 * t8369;
    let t42383 = 0.11974241701863808564e0_f64 * t8405;
    let t42384 = 0.17961362552795712846e0_f64 * t8408;
    let t42385 = 0.35922725105591425692e0_f64 * t8411;
    let t42386 = 0.11974241701863808564e0_f64 * t8414;
    let t42387 = -t8034 + t8035 + 0.72732431077987577948e-1_f64 * t7270 + t8037 + 0.2909297243119503118e0_f64 * t7280 + t8039 - 0.21819729323396273384e0_f64 * t7289 + t42383 - t42384 + t42385 - t42386;
    (t42373, t42374, t42375, t42376, t42377, t42378, t42387)
}
