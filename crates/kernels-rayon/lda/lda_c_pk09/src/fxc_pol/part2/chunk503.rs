//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 503/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk503(t2791: f64, t2859: f64, t1802: f64, t1830: f64, t1834: f64, t1840: f64, t1855: f64, t1879: f64, t1929: f64, t1933: f64, t1952: f64, t2769: f64, t2772: f64, t2779: f64, t2817: f64, t444: f64, t455: f64, t552: f64) -> (f64, f64) {
    let t2860 = t2791 + t2859;
    let t2863 = 1.8805371096875316_f64 * t2769 * t455 - 3.7610742193750633_f64 * t2772 * t455 - 1.8805371096875316_f64 * t2779 * t552 + t444 * t2860 + t1802 - t1830 + t1834 + t1840 + t1855 - t1879 - t1929 - t1933 + t1952 - 22.07984838129906_f64 * t2817;
    (t2860, t2863)
}
