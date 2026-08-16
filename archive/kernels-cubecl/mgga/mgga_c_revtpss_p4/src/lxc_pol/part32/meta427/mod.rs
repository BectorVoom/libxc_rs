//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1511;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1512;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1513;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta427<F: Float>(t1179: F, t1188: F, t20382: F, t1196: F, t5192: F, t5202: F, t5207: F, t1189: F, t6555: F, t5181: F, t5197: F, t16988: F, t5205: F, t300: F, t6513: F, t1198: F, t16784: F, t1765: F, t20283: F, t20285: F, t20287: F, t20290: F, t20295: F, t20300: F, t20304: F, t20308: F, t20312: F, t20315: F, t20320: F, t12349: F, t12352: F, t16708: F, t16893: F, t16929: F, t16931: F, t20366: F, t20368: F, t20371: F, t20373: F, t20378: F, t12261: F, t12297: F, t16706: F, t16869: F, t16873: F, t16876: F, t20268: F, t20274: F, t20276: F, t20278: F, t20280: F, t20338: F, t20341: F, t20344: F, t20347: F, t20350: F, t20353: F, t20357: F, t20359: F, t20362: F, t1150: F, t1131: F, t12243: F, t6474: F, t3531: F, t6548: F, t12382: F, t16797: F, t16798: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20386, t20388, t20390, t20393, t20396, t20397) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1511::<F>(t1179, t1188, t20382, t1196, t5192, t5202, t5207, t1189, t6555, t5181, t5197, t16988, t5205);
        let (t20399, t20402, t20404, t20425) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1512::<F>(t1196, t20397, t300, t6513, t1198, t16784, t1765, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
        let t20447 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1513::<F>(t12349, t12352, t16708, t16893, t16929, t16931, t20366, t20368, t20371, t20373, t20378, t12261, t12297, t16706, t16869, t16873, t16876, t20268, t20274, t20276, t20278, t20280, t20338, t20341, t20344, t20347, t20350, t20353, t20357, t20359, t20362, t20425);
        let (t20450, t20452, t20454, t20469) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1514::<F>(t1150, t20447, t1131, t12243, t6474, t3531, t6548, t12297, t12382, t16706, t16708, t16797, t16798, t20283, t20285, t20287, t20290, t20295, t20300, t20304, t20308, t20312, t20315, t20320);
    (t20386, t20388, t20390, t20393, t20396, t20399, t20402, t20404, t20450, t20452, t20454, t20469)
}
