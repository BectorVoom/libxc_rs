//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2220;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2221;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta493<F: Float>(t15936: F, t16208: F, t1042: F, t3124: F, t4820: F, t1655: F, t697: F, t1011: F, t372: F, t4806: F, t15702: F, t15688: F, t3299: F, t1043: F, t905: F, t606: F, t3155: F, t15691: F, t1047: F, t1063: F, t11656: F, t11977: F, t15700: F, t16190: F, t16196: F, t16201: F, t16205: F, t1671: F, t3169: F, t4825: F, t4869: F, t15625: F, t15676: F, t15722: F, t15755: F, t15779: F, t15814: F, t15855: F, t15913: F, t15949: F, t15991: F, t16034: F, t16073: F, t16114: F, t16136: F, t16189: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16209, t16210, t16218, t16219, t16220, t16222, t16223, t16226) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2220::<F>(t15936, t16208, t1042, t3124, t4820, t1655, t697, t1011, t372, t4806, t15702, t15688, t3299);
        let (t16228, t16229, t16230, t16233) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2221::<F>(t1043, t905, t606, t3155, t15691, t1047, t1063, t11656, t11977, t15700, t16190, t16196, t16201, t16205, t16210, t16218, t16220, t16223, t16226, t1671, t3169, t4825, t4869);
        let t16237 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2222::<F>(t15625, t15676, t15722, t15755, t15779, t15814, t15855, t15913, t15949, t15991, t16034, t16073, t16114, t16136, t16189, t16233);
    (t16209, t16210, t16218, t16219, t16220, t16222, t16223, t16226, t16228, t16229, t16230, t16237)
}
