//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1162/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1162(t125322: f64, t640: f64, t8621: f64, t33624: f64, t6972: f64, t1497: f64, t36: f64, t606: f64, t1518: f64, t6982: f64, t1931: f64, t4292: f64) -> (f64, f64, f64, f64, f64) {
    let t125324 = t8621 * t125322 * t640;
    let t125332 = t8621 * t33624 * t6972;
    let t125335 = t1497 * t36;
    let t125336 = t125335 * t606;
    let t125362 = t6982 * t1518;
    let t125365 = t1931 * t4292;
    (t125324, t125332, t125336, t125362, t125365)
}
