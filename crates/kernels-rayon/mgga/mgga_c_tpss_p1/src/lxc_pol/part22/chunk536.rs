//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 536/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk536(t45: f64, t57: f64, t2222: f64, t730: f64, t200: f64, t1985: f64, t1992: f64, t78: f64, t202: f64, t81: f64, t162: f64, t187: f64, t150: f64, t190: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2224 = 0.24415263074675393405e-3_f64 * t730 * t2222;
    let t2225 = 1.0_f64 / t200;
    let t2231 = piecewise3(t151, 0.0_f64, 4.0_f64 / 9.0_f64 * t2225 * t1985 + 4.0_f64 / 3.0_f64 * t78 * t1992);
    let t2232 = 1.0_f64 / t202;
    let t2238 = piecewise3(t155, 0.0_f64, 4.0_f64 / 9.0_f64 * t2232 * t1985 - 4.0_f64 / 3.0_f64 * t81 * t1992);
    let t2239 = t2231 + t2238;
    let t2240 = t2239 * t162;
    let t2242 = 0.19751673498613801407e-1_f64 * t2240 * t187;
    let t2243 = t150 * t2239;
    let t2244 = t2243 * t190;
    (t2224, t2225, t2232, t2239, t2240, t2242, t2243, t2244)
}
