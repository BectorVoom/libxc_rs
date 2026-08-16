//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1164/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1164(t3880: f64, t937: f64, t2393: f64, t10365: f64, t2464: f64, t10414: f64, t16111: f64, t440: f64, t1429: f64, t3314: f64, t8: f64, t3318: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t28492 = t937 * t3880;
    let t28493 = t2393 * t28492;
    let t28595 = t10365 * t2464;
    let t28649 = t16111 * t10414 * t440;
    let t28653 = t3314 * t8 * t1429;
    let t28658 = t973 * t3318;
    (t28493, t28595, t28649, t28653, t28658)
}
