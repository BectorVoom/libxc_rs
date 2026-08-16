//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1255/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1255(t81398: f64, t12438: f64, t12444: f64, t22653: f64, t22656: f64, t22905: f64, t3758: f64, t3882: f64, t3889: f64, t539: f64, t568: f64, t6958: f64, t6993: f64, t81011: f64, t81379: f64, t81386: f64, t81393: f64, t81395: f64) -> f64 {
    let t81399 = 0.13707783890401886971e-2_f64 * t81398;
    let t81404 = -0.24674011002723396548e-1_f64 * t81379 - 6.0_f64 * t12444 * t6993 + 0.49348022005446793095e-1_f64 * t81386 + t539 * t81011 * t568 - t6958 * t12438 + 6.0_f64 * t22656 * t3889 - 0.11514538467937585055e0_f64 * t81393 + 0.11514538467937585055e0_f64 * t81395 - t81399 - 3.0_f64 * t3882 * t22905 + 12.0_f64 * t3758 * t22653;
    t81404
}
