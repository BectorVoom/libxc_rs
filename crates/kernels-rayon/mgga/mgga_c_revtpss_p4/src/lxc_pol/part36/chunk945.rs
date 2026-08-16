//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 945/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk945(t1504: f64, t5895: f64, t10227: f64, t4269: f64, t5823: f64, t580: f64, t9342: f64, t100: f64, t5842: f64, t1509: f64, t5907: f64, t10241: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22596 = t5895 * t1504;
    let t22597 = t10227 * t22596;
    let t22600 = t4269 * t5823;
    let t22603 = -t580 - t9342;
    let t22604 = 3.0_f64 * t22603;
    let t22605 = t100 * t22604;
    let t22608 = tau1 * t5842;
    let t22617 = t5907 * t1509;
    let t22618 = t10241 * t22617;
    (t22597, t22600, t22603, t22604, t22605, t22608, t22618)
}
