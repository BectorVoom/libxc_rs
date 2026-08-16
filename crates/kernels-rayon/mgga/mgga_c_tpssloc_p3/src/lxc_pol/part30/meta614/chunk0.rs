//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2012/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2012(t3186: f64, t83015: f64, t3158: f64, t6712: f64, t10383: f64, t1926: f64, t10948: f64, t23536: f64, t10472: f64, t10474: f64, t10478: f64, t23535: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83016 = t3186 * t83015;
    let t83025 = t6712 * t3158;
    let t83028 = 5.0_f64 / 1296.0_f64 * t1926 * t10383;
    let t83043 = t10948 * t23536;
    let t83054 = t10472 * t10474 * sigma0 * t10478;
    let t83058 = t10472 * t23535 * t10478;
    (t83016, t83025, t83028, t83043, t83054, t83058)
}
