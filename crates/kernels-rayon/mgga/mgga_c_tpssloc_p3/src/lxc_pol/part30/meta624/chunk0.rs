//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2024/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2024(t23164: f64, t23204: f64, t25341: f64, t1887: f64, t81956: f64, t25041: f64, t215: f64, t6581: f64, t252: f64, t81613: f64, t23056: f64, t25242: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87028 = t23164 * t23204 * t25341;
    let t87029 = 0.16449340668482264365e-1_f64 * t87028;
    let t87049 = t81956 * t1887;
    let t87050 = t87049 * t25041;
    let t87052 = t6581 * t215;
    let t87053 = t81613 * t252;
    let t87057 = t23056 * t215;
    let t87066 = t6579 * t25242;
    (t87029, t87049, t87050, t87052, t87053, t87057, t87066)
}
