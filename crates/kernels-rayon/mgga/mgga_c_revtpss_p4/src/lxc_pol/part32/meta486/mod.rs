//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1732;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1733;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta486(t28108: f64, t77: f64, t1470: f64, t2242: f64, t4181: f64, t603: f64, t4187: f64, t1493: f64, t644: f64, t4173: f64, t607: f64, t7705: f64, t1497: f64, t1927: f64, t2247: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28109, t28112, t28116, t28119, t28133, t28141, t28147) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1732(t28108, t77, t1470, t2242, t4181, t603, t4187, t1493, t644, t4173, t607, t7705);
        let (t28150, t28154) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1733(t1497, t1927, t1470, t2247);
    (t28109, t28112, t28116, t28119, t28133, t28141, t28147, t28150, t28154)
}
