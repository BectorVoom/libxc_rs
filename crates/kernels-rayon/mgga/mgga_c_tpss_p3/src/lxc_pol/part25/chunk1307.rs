//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1307/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1307(t38: f64, t42690: f64, t13442: f64, t76: f64, t4622: f64, t619: f64, t77: f64, t13546: f64, t94: f64, t13866: f64, t1705: f64, t935: f64) -> (f64, f64, f64, f64, f64) {
    let t69281 = t42690 * t38;
    let t69338 = t76 * t13442;
    let t69355 = t77 * t4622 * t619;
    let t69383 = t94 * t13546;
    let t69452 = t1705 * t13866 * t935;
    (t69281, t69338, t69355, t69383, t69452)
}
