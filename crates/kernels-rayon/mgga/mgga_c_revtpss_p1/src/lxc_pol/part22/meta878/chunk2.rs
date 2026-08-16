//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3047/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3047(t136: f64, t2457: f64, t2710: f64, t4469: f64, t2722: f64, t50474: f64, t2782: f64, t39597: f64, t14586: f64, t10529: f64, t10115: f64, t1576: f64) -> (f64, f64, f64, f64) {
    let t51564 = t2710 * t4469 * t136 * t2457;
    let t51570 = t50474 * t2722;
    let t51572 = t2782 * t39597 * t51570;
    let t51574 = t14586 * t2722;
    let t51576 = t2782 * t10529 * t51574;
    let t51578 = t10115 * t1576;
    (t51564, t51572, t51576, t51578)
}
