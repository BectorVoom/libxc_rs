//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 748/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk748(t1375: f64, t1386: f64, t2016: f64, t3758: f64, t3882: f64, t568: f64, t6885: f64, t6893: f64, t6900: f64, t6904: f64, t6909: f64, t6911: f64, t6956: f64, t6958: f64, t6963: f64, t6993: f64) -> f64 {
    let t6995 = -t6885 - 0.16449340668482264365e-1_f64 * t6893 - t6900 + 0.82246703342411321825e-2_f64 * t6904 - 0.82246703342411321825e-2_f64 * t6909 + t6911 * t568 + t6956 * t568 - t6958 * t1386 - t3758 * t2016 - t3882 * t2016 + 2.0_f64 * t1375 * t6963 - t1375 * t6993;
    t6995
}
