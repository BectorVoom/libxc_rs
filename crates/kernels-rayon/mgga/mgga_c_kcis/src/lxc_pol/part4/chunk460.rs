//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 460/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk460(t1260: f64, t1851: f64, t286: f64, t1249: f64, t1251: f64, t1847: f64, t1268: f64, t1240: f64, t1272: f64, t1715: f64, t1751: f64, t1770: f64, t1776: f64, t1844: f64, t430: f64) -> (f64, f64, f64, f64, f64) {
    let t1852 = t1260 * t1851;
    let t1853 = t286 * t1852;
    let t1856 = t1249 + t1251 * t1847 / 576.0_f64 - t1251 * t1853 / 192.0_f64;
    let t1857 = t1856 * t1268;
    let t1864 = t1844 * t430 - 0.66725e-1_f64 * t1240 * t1857 + t1272 + 0.11607361111111111111e-2_f64 * t1715 + 0.17411041666666666666e-2_f64 * t1751 - 0.17411041666666666666e-2_f64 * t1770 + 0.11607361111111111111e-2_f64 * t1776;
    (t1852, t1853, t1856, t1857, t1864)
}
