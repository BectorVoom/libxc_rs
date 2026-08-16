//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1035/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1035(t1375: f64, t1386: f64, t1843: f64, t3758: f64, t3882: f64, t5211: f64, t5213: f64, t5215: f64, t5217: f64, t5319: f64, t5321: f64, t5326: f64, t5354: f64, t568: f64) -> f64 {
    let t5356 = 2.0_f64 * t1375 * t5326 - t1375 * t5354 - t1386 * t5215 - t1386 * t5321 - t1843 * t3758 - t1843 * t3882 + t5211 * t568 + t5213 * t568 + t5217 * t568 + t5319 * t568;
    t5356
}
