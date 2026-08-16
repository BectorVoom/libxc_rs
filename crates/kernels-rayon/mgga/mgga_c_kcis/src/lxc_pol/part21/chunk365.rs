//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 365/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk365(t1268: f64, t1856: f64, t1240: f64, t1272: f64, t1715: f64, t1751: f64, t1770: f64, t1776: f64, t1844: f64, t430: f64, t1798: f64, t1802: f64, t1806: f64, t1810: f64, t1814: f64, t1818: f64) -> (f64, f64, f64) {
    let t1857 = t1856 * t1268;
    let t1864 = t1844 * t430 - 0.66725e-1_f64 * t1240 * t1857 + t1272 + 0.11607361111111111111e-2_f64 * t1715 + 0.17411041666666666666e-2_f64 * t1751 - 0.17411041666666666666e-2_f64 * t1770 + 0.11607361111111111111e-2_f64 * t1776;
    let t1872 = 0.9375e-1_f64 * t1798 - 0.9375e-1_f64 * t1802 + 0.625e-1_f64 * t1806 - 0.101171875e-1_f64 * t1810 + 0.101171875e-1_f64 * t1814 - 0.13489583333333333333e-1_f64 * t1818;
    (t1857, t1864, t1872)
}
