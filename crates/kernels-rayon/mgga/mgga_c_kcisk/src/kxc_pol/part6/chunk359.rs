//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 359/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk359(t2257: f64, t467: f64, t488: f64, t2231: f64, t470: f64, t487: f64, t1487: f64, t2152: f64, t382: f64, t486: f64, t2211: f64, t492: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2258 = t2257 * t467;
    let t2259 = t2258 * sigma0;
    let t2260 = t2259 * t488;
    let t2262 = t470 * t2231;
    let t2263 = t487 * t2262;
    let t2264 = t1487 * t2263;
    let t2266 = t382 * t2152;
    let t2267 = t487 * t2266;
    let t2268 = t486 * t2267;
    let t2270 = t2211 * t467;
    let t2271 = t2270 * t492;
    (t2259, t2260, t2263, t2264, t2267, t2268, t2270, t2271)
}
