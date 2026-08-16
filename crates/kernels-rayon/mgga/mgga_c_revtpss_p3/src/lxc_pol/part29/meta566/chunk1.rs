//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1913/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1913(t7234: f64, t8995: f64, t14468: f64, t30: f64, t2: f64, t2411: f64, t580: f64, t890: f64, t892: f64, t775: f64, t1583: f64, t2430: f64) -> (f64, f64, f64, f64, f64) {
    let t98588 = t7234 * t8995;
    let t98627 = t30 * t14468;
    let t98631 = t2411 * t2;
    let t98633 = t98631 * t580 * t890;
    let t98646 = t892 * t2;
    let t98648 = t98646 * t580 * t775;
    let t98651 = t1583 * t2430;
    (t98588, t98627, t98633, t98648, t98651)
}
