//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1305/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1305(t1449: f64, t666: f64, t8184: f64, t662: f64, t30063: f64, t2: f64, t29903: f64, t30048: f64, t30049: f64, t30051: f64, t30175: f64, t30279: f64, t30281: f64, t30285: f64, t30288: f64, t30291: f64, t30294: f64, t30298: f64, t30301: f64, t8128: f64, t8137: f64) -> (f64, f64, f64, f64) {
    let t30303 = t1449 * t666;
    let t30304 = t8184 * t30303;
    let t30307 = t1449 * t662;
    let t30308 = t30063 * t30307;
    let t30311 = t8184 * t2;
    let t30314 = -t30048 - 2.0_f64 / 3.0_f64 * t30049 + 5.0_f64 / 9.0_f64 * t30051 - 2.0_f64 / 3.0_f64 * t30279 - 3.0_f64 / 4.0_f64 * t29903 * t30281 + 5.0_f64 / 12.0_f64 * t8128 * t30285 + t8128 * t30288 / 4.0_f64 - 5.0_f64 / 9.0_f64 * t30291 - 5.0_f64 / 12.0_f64 * t8128 * t30294 + 25.0_f64 / 72.0_f64 * t8137 * t30298 + 5.0_f64 / 9.0_f64 * t30301 + 5.0_f64 / 12.0_f64 * t8128 * t30304 - 5.0_f64 / 36.0_f64 * t8137 * t30308 + 5.0_f64 / 24.0_f64 * t30175 * t30311;
    (t30304, t30308, t30311, t30314)
}
