//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 522/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk522(t650: f64, t713: f64, t182: f64, t712: f64, t177: f64, t2211: f64, t720: f64, t2204: f64, t2210: f64, t2214: f64, t123: f64, t173: f64, t186: f64, t2192: f64, t2250: f64, t2256: f64, t2258: f64, t2268: f64, t2273: f64, t2276: f64, t2281: f64, t2285: f64, t2292: f64, t2302: f64, t2310: f64, t262: f64, t699: f64, t706: f64, t714: f64, t721: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2314 = t650 * t713;
    let t2318 = t712 * t182;
    let t2319 = 1.0_f64 / t2318;
    let t2320 = t177 * t2319;
    let t2321 = t2211 * t720;
    let t2324 = t2204 * t720;
    let t2327 = t177 * t2210;
    let t2328 = t2211 * t2214;
    let t2331 = -0.70983522622222222221e-3_f64 * t123 * t2192 * t173 - 0.34246666666666666666e-1_f64 * t262 * t2250 * t706 - 2.0_f64 * t2256 * t2258 + 1.0_f64 * t699 * t2268 + 0.32163958997385070134e2_f64 * t2273 * t2276 + t2281 + t2285 + t2292 - t2302 - t2310 - 0.24415263074675393405e-3_f64 * t123 * t2192 * t186 - 0.10843581300301739842e-1_f64 * t262 * t2314 * t721 - 0.11696447245269292414e1_f64 * t2320 * t2321 + 0.5848223622634646207e0_f64 * t714 * t2324 + 0.17315859105681463759e2_f64 * t2327 * t2328;
    (t2314, t2319, t2320, t2321, t2324, t2327, t2328, t2331)
}
