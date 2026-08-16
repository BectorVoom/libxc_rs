//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1526/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1526(t1063: f64, t11725: f64, t23481: f64, t247: f64, t23474: f64, t3109: f64, t23847: f64, t3127: f64, t3172: f64, t23858: f64, t23634: f64, t1065: f64, t24031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78550 = t1063 * t247 * t11725 * t23481;
    let t78561 = t1063 * t247 * t3109 * t23474;
    let t78564 = t3127 * t3172 * t23847;
    let t78576 = t3127 * t3172 * t23858;
    let t78583 = t3127 * t3172 * t23634;
    let t78607 = t1065 * t24031;
    (t78550, t78561, t78564, t78576, t78583, t78607)
}
