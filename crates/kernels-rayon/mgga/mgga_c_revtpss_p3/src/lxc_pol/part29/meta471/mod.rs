//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1734;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1735;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1736;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta471(t25310: f64, t7407: f64, t25305: f64, t26519: f64, t26506: f64, t7058: f64, t2471: f64, t7388: f64, t25375: f64, t26485: f64, t72: f64, t7423: f64, t686: f64, t213: f64, t7398: f64, t2061: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26529, t26534, t26536, t26538, t26541, t26543) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1734(t25310, t7407, t25305, t26519, t26506, t7058, t2471, t7388, t25375, t26485, t72, t7423);
        let t26544 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1735(t26543, t686);
        let (t26545, t26547, t26550) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1736(t26544, t7058, t213, t7398, t2061, t822);
    (t26529, t26534, t26536, t26538, t26541, t26543, t26544, t26545, t26547, t26550)
}
