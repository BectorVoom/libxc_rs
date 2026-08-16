//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1249/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1249(t100966: f64, t100972: f64, t103091: f64, t103092: f64, t103098: f64, t103099: f64, t108858: f64, t108871: f64, t1398: f64, t1852: f64, t1858: f64, t2099: f64, t2105: f64, t22431: f64, t22453: f64, t29396: f64, t29430: f64, t3: f64, t580: f64, t6471: f64, t6483: f64, t7946: f64, t7961: f64) -> f64 {
    let tv4rho3sigma10 = t108858 * t3 * t580 + t108871 * t1398 + 3.0_f64 * t1852 * t29430 + 3.0_f64 * t1858 * t29396 + t2099 * t22453 + t2105 * t22431 + 3.0_f64 * t6471 * t7961 + 3.0_f64 * t6483 * t7946 + 3.0_f64 * t100966 + 6.0_f64 * t100972 + 3.0_f64 * t103091 + 6.0_f64 * t103092 + 3.0_f64 * t103098 + 3.0_f64 * t103099;
    tv4rho3sigma10
}
