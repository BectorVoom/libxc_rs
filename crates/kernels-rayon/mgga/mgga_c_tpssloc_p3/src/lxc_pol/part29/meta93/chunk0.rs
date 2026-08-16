//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 607/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk607(t2002: f64, t544: f64, t559: f64, t1992: f64, t2000: f64) -> (f64, f64) {
    let t2003 = t544 * t2002;
    let t2004 = t2003 * t559;
    let t2006 = t1992 / 96.0_f64 + 0.20186378047070195427e-3_f64 * t2000 + t2004 / 1536.0_f64;
    (t2003, t2006)
}
