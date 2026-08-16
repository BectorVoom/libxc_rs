//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1012 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1012(t15547: f64, t4725: f64, t1642: f64, t52921: f64, t4729: f64, t4734: f64, t64465: f64, t64467: f64, t64471: f64, t64475: f64, t64483: f64, t65402: f64, t65404: f64, t65408: f64, t65413: f64, t65388: f64, t65389: f64, t65391: f64, t65392: f64, t65395: f64, t65396: f64, t65398: f64, t19658: f64, t3169: f64, t13312: f64, t1469: f64, t1041: f64, t1042: f64, t1045: f64, t1063: f64, t11268: f64, t16208: f64, t19668: f64, t19675: f64, t247: f64, t2862: f64, t3127: f64, t3182: f64, t3188: f64, t373: f64, t42943: f64, t4806: f64, t6302: f64, t6312: f64, t63455: f64, t65357: f64, t65359: f64, t65365: f64, t65370: f64, t65376: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t65415, t65417, t65419, t65421, t65422) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3476(t15547, t4725, t1642, t52921, t4729, t4734, t64465, t64467, t64471, t64475, t64483, t65402, t65404, t65408, t65413);
        let (t65425, t65431, t65433) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3477(t65388, t65389, t65391, t65392, t65395, t65396, t65398, t65422, t19658, t3169, t13312, t1469);
        let t65438 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3478(t1041, t1042, t1045, t1063, t11268, t16208, t19668, t19675, t247, t2862, t3127, t3182, t3188, t373, t42943, t4806, t6302, t6312, t63455, t65357, t65359, t65365, t65370, t65376, t65425, t65431, t65433);
    (t65415, t65417, t65419, t65421, t65425, t65433, t65438)
}
