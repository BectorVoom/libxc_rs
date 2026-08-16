//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1278/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1278(t1375: f64, t1843: f64, t2016: f64, t5215: f64, t5321: f64, t568: f64, t6885: f64, t6900: f64, t6958: f64, t7693: f64, t7698: f64, t7702: f64, t7704: f64, t7723: f64, t7729: f64, t7750: f64) -> f64 {
    let t7752 = -t6885 - 0.16449340668482264365e-1_f64 * t7693 - t6900 + 0.82246703342411321825e-2_f64 * t7698 - 0.82246703342411321825e-2_f64 * t7702 + t7704 * t568 + t7723 * t568 - t6958 * t1843 - t5215 * t2016 - t5321 * t2016 + 2.0_f64 * t1375 * t7729 - t1375 * t7750;
    t7752
}
