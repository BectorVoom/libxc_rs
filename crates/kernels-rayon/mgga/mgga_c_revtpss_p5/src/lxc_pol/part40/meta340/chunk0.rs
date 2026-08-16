//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1141/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1141(t100: f64, t580: f64, t22: f64, t4273: f64, t10241: f64, t1509: f64, t2358: f64, t105: f64, t2357: f64, t2255: f64, t661: f64, t2362: f64, t4279: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13482 = t100 * t580;
    let t13485 = t4273 * t22;
    let t13493 = t10241 * t1509 * t2358;
    let t13496 = t105 * t2357;
    let t13497 = t2255 * t661;
    let t13500 = t4279 * t2362;
    (t13482, t13485, t13493, t13496, t13497, t13500)
}
