//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 537/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk537(t692: f64, t725: f64, t650: f64, t698: f64, t169: f64, t697: f64, t164: f64, t704: f64, t705: f64, t2187: f64, t2190: f64, t2193: f64, t2197: f64, t2199: f64, t2202: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2245 = t692 * t725;
    let t2246 = 2.0_f64 * t2245;
    let t2250 = t650 * t698;
    let t2254 = t697 * t169;
    let t2255 = 1.0_f64 / t2254;
    let t2256 = t164 * t2255;
    let t2257 = t704 * t704;
    let t2258 = t2257 * t705;
    let t2267 = -0.78438333333333333333e0_f64 * t2187 + 0.15687666666666666667e1_f64 * t2190 + 0.68863333333333333333e0_f64 * t2193 + 0.14025833333333333333e0_f64 * t2197 + 0.28051666666666666667e0_f64 * t2199 + 0.17365833333333333333e0_f64 * t2202;
    (t2245, t2246, t2250, t2255, t2256, t2257, t2258, t2267)
}
