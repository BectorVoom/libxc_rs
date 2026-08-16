//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 585/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk585(t2453: f64, t942: f64, t2422: f64, t2430: f64, t411: f64, t415: f64, t938: f64, t952: f64) -> (f64, f64) {
    let t2454 = t942 * t2453;
    let t2457 = 0.65854491829355115987e0_f64 * t2422 * t415 - 0.13170898365871023197e1_f64 * t938 * t952 + 0.13170898365871023197e1_f64 * t411 * t2430 - 0.65854491829355115987e0_f64 * t411 * t2454;
    (t2454, t2457)
}
