//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1417/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1417(t2662: f64, t268: f64, t40689: f64, t4353: f64, t40710: f64, t4349: f64, t1558: f64, t231: f64, t40406: f64, t685: f64, t72: f64, t826: f64) -> (f64, f64, f64) {
    let t50381 = t40689 * t2662 * t4353 * t268;
    let t50385 = t40710 * t4349;
    let t50436 = t40406 * t826 * t1558 * t231 * t72 * t685;
    (t50381, t50385, t50436)
}
