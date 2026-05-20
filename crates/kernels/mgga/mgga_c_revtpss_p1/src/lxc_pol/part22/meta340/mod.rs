//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1809;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1810;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta340<F: Float>(t11249: F, t3154: F, t246: F, t676: F, t1046: F, t1041: F, t1038: F, t3229: F, t1036: F, t1033: F, t3169: F, t3173: F, t3140: F, t989: F, t3149: F, t3160: F, t2866: F, t914: F, t2923: F, t910: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11250, t11262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1809::<F>(t11249, t3154, t246, t676);
        let (t11263, t11264, t11267, t11268, t11271, t11273, t11274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1810::<F>(t1046, t11262, t1041, t1038, t3229, t1036, t1033, t3169, t3173, t3140, t989, t3149);
        let (t11277, t11289, t11294) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1811::<F>(t11273, t3160, t2866, t914, t2923, t910);
    (t11250, t11262, t11263, t11264, t11267, t11268, t11271, t11273, t11274, t11277, t11289, t11294)
}
