//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 407/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk407(t1859: f64, t423: f64, t170: f64, t591: f64, t597: f64, t584: f64, t608: f64, t1399: f64, t1714: f64, t1717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1860 = t1859 * t423;
    let t1861 = t170 * t591;
    let t1862 = t597 * t1861;
    let t1863 = t1860 * t1862;
    let t1866 = t584 * t608 * t591;
    let t1870 = 0.13949e-1_f64 * t1399;
    let t1871 = -0.24694444444444444445e-2_f64 * t1714 + 0.19755555555555555556e-1_f64 * t1717 + t1870;
    (t1860, t1861, t1862, t1863, t1866, t1870, t1871)
}
