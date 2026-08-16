//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1997;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta584(t2453: f64, t25309: f64, t25301: f64, t25304: f64, t251: f64, t25410: f64, t136: f64, t137: f64, t1949: f64, t2438: f64, t837: f64, t25305: f64, t92894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93158, t93161, t93169, t93170, t93172, t93174, t93175, t93177) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1997(t2453, t25309, t25301, t25304, t251, t25410, t136, t137, t1949, t2438, t837, t25305, t92894);
    (t93158, t93161, t93169, t93170, t93172, t93174, t93175, t93177)
}
