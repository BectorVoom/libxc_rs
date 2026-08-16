//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 678/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk678(t237: f64, t3143: f64, t3114: f64, t1201: f64) -> (f64, f64, f64) {
    let t3144 = t237 * t3143;
    let t3146 = 0.19751673498613801407e-1_f64 * t237 * t3114;
    let t3147 = t237 * t1201;
    (t3144, t3146, t3147)
}
