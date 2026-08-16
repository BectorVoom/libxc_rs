//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1762/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1762(t22940: f64, t22870: f64, t539: f64, t12033: f64, t1375: f64, t2016: f64, t22688: f64, t22905: f64, t22908: f64, t22910: f64, t22913: f64, t22918: f64, t22922: f64, t22924: f64, t22926: f64, t22928: f64, t22931: f64, t22936: f64, t3758: f64, t3889: f64, t568: f64, t6958: f64, t6963: f64, t6993: f64) -> (f64, f64) {
    let t22941 = 0.38381794893125283518e-1_f64 * t22940;
    let t22942 = t539 * t22870;
    let t22946 = 2.0_f64 * t6958 * t3889 + 0.49348022005446793095e-1_f64 * t22688 - t1375 * t22905 + t22908 + t22910 - t12033 * t2016 + 2.0_f64 * t1375 * t22913 - 0.16449340668482264365e-1_f64 * t22918 + t22922 + t22924 + t22926 - 0.82246703342411321824e-2_f64 * t22928 - 0.3289868133696452873e-1_f64 * t22931 + 0.16449340668482264365e-1_f64 * t22936 + 4.0_f64 * t3758 * t6963 - t22941 + t22942 * t568 - 2.0_f64 * t3758 * t6993;
    (t22942, t22946)
}
