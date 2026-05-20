//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1557;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1558;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1559;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1560;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta436<F: Float>(t15957: F, t6266: F, t3092: F, t16509: F, t4891: F, t16584: F, t1045: F, t19497: F, t3117: F, t1043: F, t11631: F, t19450: F, t4894: F, t19501: F, t4910: F, t11274: F, t11277: F, t11789: F, t11875: F, t15684: F, t15906: F, t16081: F, t3091: F, t3115: F, t4896: F, t4902: F, t6308: F, t6312: F, t6339: F, t19380: F, t373: F, t371: F, t372: F, t19463: F, t366: F, t3094: F, t4186: F, t4781: F, t4786: F, t6092: F, t11703: F, t11710: F, t6267: F, t4583: F, t4823: F, t1042: F, t1025: F, t1028: F, t15618: F, t15712: F, t15724: F, t3124: F, t3127: F, t3224: F, t4788: F, t6278: F, t6302: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19731, t19738, t19741, t19745, t19749) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1557::<F>(t15957, t6266, t3092, t16509, t4891, t16584, t1045, t19497, t3117, t1043, t11631, t19450);
        let (t19750, t19754, t19758, t19763) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1558::<F>(t19749, t3117, t19450, t4894, t19501, t4910, t11274, t11277, t11789, t11875, t15684, t15906, t16081, t19731, t19738, t19741, t19745, t3091, t3115, t4896, t4902, t6308, t6312, t6339);
        let (t19770, t19773, t19778, t19781) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1559::<F>(t19380, t373, t371, t372, t19463, t366, t3094, t4186, t4781, t3092, t4786, t6092);
        let (t19782, t19785, t19792, t19797) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1560::<F>(t11703, t19781, t11710, t6267, t3091, t4583, t4823, t1042, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t3124, t3127, t3224, t4788, t6278, t6302);
    (t19731, t19745, t19750, t19754, t19758, t19763, t19770, t19778, t19782, t19785, t19792, t19797)
}
