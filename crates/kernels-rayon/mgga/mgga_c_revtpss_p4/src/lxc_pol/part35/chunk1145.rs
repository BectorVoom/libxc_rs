//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1145/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1145(t2435: f64, t28825: f64, t1904: f64, t2439: f64, t26358: f64, t10073: f64, t25937: f64, t7282: f64, t8085: f64, t102385: f64, t94383: f64, t26260: f64, t27836: f64) -> (f64, f64, f64, f64, f64) {
    let t102462 = t2435 * t28825;
    let t102582 = t2439 * t26358 * t1904;
    let t102610 = t10073 * t7282 * t25937 * t8085;
    let t102629 = t94383 * t102385;
    let t102636 = t10073 * t27836 * t26260;
    (t102462, t102582, t102610, t102629, t102636)
}
