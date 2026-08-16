//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1557;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1558;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1559;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta436(t15957: f64, t6266: f64, t3092: f64, t16509: f64, t4891: f64, t16584: f64, t1045: f64, t19497: f64, t3117: f64, t1043: f64, t11631: f64, t19450: f64, t4894: f64, t19501: f64, t4910: f64, t11274: f64, t11277: f64, t11789: f64, t11875: f64, t15684: f64, t15906: f64, t16081: f64, t3091: f64, t3115: f64, t4896: f64, t4902: f64, t6308: f64, t6312: f64, t6339: f64, t19380: f64, t373: f64, t371: f64, t372: f64, t19463: f64, t366: f64, t3094: f64, t4186: f64, t4781: f64, t4786: f64, t6092: f64, t11703: f64, t11710: f64, t6267: f64, t4583: f64, t4823: f64, t1042: f64, t1025: f64, t1028: f64, t15618: f64, t15712: f64, t15724: f64, t3124: f64, t3127: f64, t3224: f64, t4788: f64, t6278: f64, t6302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19731, t19738, t19741, t19745, t19749) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1557(t15957, t6266, t3092, t16509, t4891, t16584, t1045, t19497, t3117, t1043, t11631, t19450);
        let (t19750, t19754, t19758, t19763) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1558(t19749, t3117, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t19745, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
        let (t19770, t19773, t19778, t19781) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1559(t19380, t373, t371, t372, t19463, t366, t3094, t4186, t4781, t3092, t4786, t6092);
        let (t19782, t19785, t19792, t19797) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1560(t11703, t19781, t11710, t6267, t3091, t4583, t4823, t1042, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t3124, t3127, t3224, t4788, t6278, t6302);
    (t19731, t19745, t19750, t19754, t19758, t19763, t19770, t19778, t19782, t19785, t19792, t19797)
}
