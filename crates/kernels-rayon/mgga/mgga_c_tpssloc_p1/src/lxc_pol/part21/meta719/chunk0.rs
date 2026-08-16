//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2562/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2562(t2770: f64, t2987: f64, t10277: f64, t4509: f64, t10390: f64, t13765: f64, t10937: f64, t14501: f64, t1606: f64, t2402: f64, t973: f64, t10454: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50366 = t2987 * t2770;
    let t50370 = t4509 * t10277;
    let t50378 = t10390 * t13765;
    let t50384 = t10937 * t14501;
    let t50425 = t973 * t2402 * t1606;
    let t50429 = t4644 * t10454;
    (t50366, t50370, t50378, t50384, t50425, t50429)
}
