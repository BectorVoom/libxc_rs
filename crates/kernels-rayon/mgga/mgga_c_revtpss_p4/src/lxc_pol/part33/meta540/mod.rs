//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1911;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta540(t1794: f64, t2142: f64, t73: f64, t1203: f64, t5457: f64, t5458: f64, t1294: f64, t5215: f64, t7637: f64, t1828: f64, t7627: f64, t7652: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t29157, t29159, t29160, t29163, t29166, t29167, t29174, t29175, t29178, t29179) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1911(t1794, t2142, t73, t1203, t5457, t5458, t1294, t5215, t7637, t1828, t7627, t7652);
    (t29157, t29159, t29160, t29163, t29166, t29167, t29174, t29175, t29178, t29179)
}
