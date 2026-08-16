//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 847/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk847(t1707: f64, t898: f64, t1726: f64, t1727: f64, t956: f64, t2755: f64, t406: f64, t1861: f64, t2768: f64, t1860: f64, t1859: f64, t2482: f64) -> (f64, f64, f64, f64, f64) {
    let t7647 = t898 * t1707;
    let t7650 = t1726 * t956 * t1727;
    let t7653 = 8.0_f64 * t406 * t2755;
    let t7654 = t2768 * t1861;
    let t7656 = 0.2701041328e0_f64 * t1860 * t7654;
    let t7657 = t1859 * t2482;
    (t7647, t7650, t7653, t7656, t7657)
}
