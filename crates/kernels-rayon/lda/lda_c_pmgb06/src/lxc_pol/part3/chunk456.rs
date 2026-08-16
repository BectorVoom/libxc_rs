//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 456/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk456(t178: f64, t1848: f64, t513: f64, t831: f64, t432: f64, t815: f64, t350: f64, t810: f64, t1438: f64, t760: f64, t332: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1850 = t1848 * t178 / 30.0_f64;
    let t1852 = t831 * t513 / 30.0_f64;
    let t1854 = t432 * t815 / 30.0_f64;
    let t1856 = t350 * t810;
    let t1858 = t1438 * t760;
    let t1859 = t1858 * t332;
    (t1850, t1852, t1854, t1856, t1858, t1859)
}
