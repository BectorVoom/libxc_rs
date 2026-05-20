//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1012 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1012<F: Float>(t15547: F, t4725: F, t1642: F, t52921: F, t4729: F, t4734: F, t64465: F, t64467: F, t64471: F, t64475: F, t64483: F, t65402: F, t65404: F, t65408: F, t65413: F, t65388: F, t65389: F, t65391: F, t65392: F, t65395: F, t65396: F, t65398: F, t19658: F, t3169: F, t13312: F, t1469: F, t1041: F, t1042: F, t1045: F, t1063: F, t11268: F, t16208: F, t19668: F, t19675: F, t247: F, t2862: F, t3127: F, t3182: F, t3188: F, t373: F, t42943: F, t4806: F, t6302: F, t6312: F, t63455: F, t65357: F, t65359: F, t65365: F, t65370: F, t65376: F) -> (F, F, F, F, F, F, F) {
        let (t65415, t65417, t65419, t65421, t65422) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476::<F>(t15547, t4725, t1642, t52921, t4729, t4734, t64465, t64467, t64471, t64475, t64483, t65402, t65404, t65408, t65413);
        let (t65425, t65431, t65433) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477::<F>(t65388, t65389, t65391, t65392, t65395, t65396, t65398, t65422, t19658, t3169, t13312, t1469);
        let t65438 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478::<F>(t1041, t1042, t1045, t1063, t11268, t16208, t19668, t19675, t247, t2862, t3127, t3182, t3188, t373, t42943, t4806, t6302, t6312, t63455, t65357, t65359, t65365, t65370, t65376, t65425, t65431, t65433);
    (t65415, t65417, t65419, t65421, t65425, t65433, t65438)
}
