//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2646/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646(t1831: f64, t40059: f64, t16336: f64, t3872: f64, t12336: f64, t12361: f64, t1363: f64, t1367: f64, t16321: f64, t16333: f64, t3783: f64, t40287: f64, t5240: f64, t5314: f64, t53856: f64, t53882: f64, t53883: f64, t53893: f64, t53895: f64, t53897: f64, t820: f64) -> f64 {
    let t53901 = t40059 * t1831;
    let t53903 = t16336 * t3872;
    let t53905 = 5.0_f64 / 256.0_f64 * t16321 * t3872 - t5240 * t12361 / 768.0_f64 - t53882 + 7.0_f64 / 384.0_f64 * t53883 - t1363 * t1367 * t820 * t53856 / 768.0_f64 - t12336 * t5314 / 256.0_f64 - t3783 * t16333 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t53893 + 7.0_f64 / 384.0_f64 * t53895 + 7.0_f64 / 192.0_f64 * t53897 - t40287 * t1831 / 768.0_f64 + 595.0_f64 / 2592.0_f64 * t53901 - 35.0_f64 / 384.0_f64 * t53903;
    t53905
}
