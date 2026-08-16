//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta554(t7284: f64, t96370: f64, t26234: f64, t94886: f64, t4132: f64, t689: f64, t7492: f64, t1445: f64, t2439: f64, t26358: f64, t26252: f64, t3920: f64, t26249: f64, t9664: f64, t25895: f64, t96264: f64, t25899: f64, t96431: f64, t26354: f64, t1426: f64, t7507: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t96550, t96552, t96556, t96559, t96561) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894(t7284, t96370, t26234, t94886, t4132, t689, t7492, t1445, t2439, t26358, t26252, t3920);
        let (t96564, t96565, t96567, t96570, t96576) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1895(t26249, t9664, t25895, t96264, t25899, t96431, t1445, t26354, t689, t1426, t7507, t786);
    (t96550, t96552, t96556, t96559, t96561, t96564, t96565, t96567, t96570, t96576)
}
