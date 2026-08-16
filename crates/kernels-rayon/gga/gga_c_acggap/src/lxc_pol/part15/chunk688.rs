//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 688/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk688(t322: f64, t355: f64, t368: f64, t7458: f64, t7457: f64, t1967: f64, t2109: f64, t2113: f64, t1988: f64, t2104: f64, t1024: f64, t19: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7459 = t355 * t322;
    let t7461 = t7458 * t368 * t7459;
    let t7462 = t7457 * t7461;
    let t7464 = t1967 * t2109;
    let t7465 = 0.37737710747524982482e-2_f64 * t7464;
    let t7466 = t1967 * t2113;
    let t7468 = t1988 * t2104;
    let t7469 = 0.15724046144802076034e-2_f64 * t7468;
    let t7475 = t1024 * t19;
    (t7459, t7461, t7462, t7465, t7466, t7469, t7475)
}
