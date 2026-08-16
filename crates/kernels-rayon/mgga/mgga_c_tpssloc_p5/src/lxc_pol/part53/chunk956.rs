//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 956/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk956(t114864: f64, t114891: f64, t112834: f64, t112840: f64, t112850: f64, t112855: f64, t225: f64, t31974: f64, t114932: f64, t114943: f64, t114672: f64, t31984: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t116557 = 0.25587863262083522346e0_f64 * t114864;
    let t116578 = 0.10417915756705434098e0_f64 * t114891;
    let t116608 = 0.84334201618871038669e-2_f64 * t112834;
    let t116610 = 0.26915170729426927235e-3_f64 * t112840;
    let t116613 = 119.0_f64 / 1728.0_f64 * t112850;
    let t116615 = 0.18086994730174895102e0_f64 * t112855;
    let t116645 = t31974 * t225;
    let t116648 = 0.3289868133696452873e-1_f64 * t114932;
    let t116654 = 0.3289868133696452873e-1_f64 * t114943;
    let t116673 = 0.10417915756705434098e0_f64 * t114672;
    let t116681 = t814 * t31984;
    (t116557, t116578, t116608, t116610, t116613, t116615, t116645, t116648, t116654, t116673, t116681)
}
