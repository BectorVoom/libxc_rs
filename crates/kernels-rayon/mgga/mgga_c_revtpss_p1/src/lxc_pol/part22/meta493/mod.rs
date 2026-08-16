//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2220;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2221;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta493(t15936: f64, t16208: f64, t1042: f64, t3124: f64, t4820: f64, t1655: f64, t697: f64, t1011: f64, t372: f64, t4806: f64, t15702: f64, t15688: f64, t3299: f64, t1043: f64, t905: f64, t606: f64, t3155: f64, t15691: f64, t1047: f64, t1063: f64, t11656: f64, t11977: f64, t15700: f64, t16190: f64, t16196: f64, t16201: f64, t16205: f64, t1671: f64, t3169: f64, t4825: f64, t4869: f64, t15625: f64, t15676: f64, t15722: f64, t15755: f64, t15779: f64, t15814: f64, t15855: f64, t15913: f64, t15949: f64, t15991: f64, t16034: f64, t16073: f64, t16114: f64, t16136: f64, t16189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16209, t16210, t16218, t16219, t16220, t16222, t16223, t16226) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2220(t15936, t16208, t1042, t3124, t4820, t1655, t697, t1011, t372, t4806, t15702, t15688, t3299);
        let (t16228, t16229, t16230, t16233) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2221(t1043, t905, t606, t3155, t15691, t1047, t1063, t11656, t11977, t15700, t16190, t16196, t16201, t16205, t16210, t16218, t16220, t16223, t16226, t1671, t3169, t4825, t4869);
        let t16237 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2222(t15625, t15676, t15722, t15755, t15779, t15814, t15855, t15913, t15949, t15991, t16034, t16073, t16114, t16136, t16189, t16233);
    (t16209, t16210, t16218, t16219, t16220, t16222, t16223, t16226, t16228, t16229, t16230, t16237)
}
