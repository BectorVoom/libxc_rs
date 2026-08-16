//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1266/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1266(t11108: f64, t1306: f64, t17753: f64, t30273: f64, t30275: f64, t30277: f64, t30362: f64, t30364: f64, t30366: f64, t30369: f64, t30379: f64, t30381: f64, t30385: f64, t803: f64) -> f64 {
    let t30998 = -6.0_f64 * t11108 * t1306 * t17753 * t803 + t30273 - t30275 - t30277 + t30362 + t30364 + t30366 + t30369 + t30379 + t30381 + t30385;
    t30998
}
