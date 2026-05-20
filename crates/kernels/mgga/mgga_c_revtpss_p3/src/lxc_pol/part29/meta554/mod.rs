//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta554<F: Float>(t7284: F, t96370: F, t26234: F, t94886: F, t4132: F, t689: F, t7492: F, t1445: F, t2439: F, t26358: F, t26252: F, t3920: F, t26249: F, t9664: F, t25895: F, t96264: F, t25899: F, t96431: F, t26354: F, t1426: F, t7507: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t96550, t96552, t96556, t96559, t96561) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1894::<F>(t7284, t96370, t26234, t94886, t4132, t689, t7492, t1445, t2439, t26358, t26252, t3920);
        let (t96564, t96565, t96567, t96570, t96576) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1895::<F>(t26249, t9664, t25895, t96264, t25899, t96431, t1445, t26354, t689, t1426, t7507, t786);
    (t96550, t96552, t96556, t96559, t96561, t96564, t96565, t96567, t96570, t96576)
}
