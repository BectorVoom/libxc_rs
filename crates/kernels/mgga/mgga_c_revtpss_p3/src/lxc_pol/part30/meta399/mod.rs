//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1499;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1500;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta399<F: Float>(t14605: F, t2482: F, t2801: F, t10443: F, t10552: F, t10554: F, t14312: F, t14313: F, t14315: F, t14317: F, t14324: F, t14327: F, t14329: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t10566: F, t10568: F, t14333: F, t14335: F, t14337: F, t14340: F, t14343: F, t14345: F, t14352: F, t14364: F, t14372: F, t14373: F, t14374: F, t14379: F, t14380: F, t9394: F, t10577: F, t10582: F, t10584: F, t10586: F, t14385: F, t14388: F, t14392: F, t14396: F, t14428: F, t14433: F, t14434: F, t9514: F, t9517: F, t9521: F, t9524: F, t1531: F, t37: F, t2612: F, t4392: F, t72: F, t757: F, t14425: F, t150: F, t190: F, t10608: F, t2258: F, t4402: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14608, t14609) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1498::<F>(t14605, t2482, t2801, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t9278, t9308, t9316, t9329, t9333);
        let t14610 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1499::<F>(t10566, t10568, t14333, t14335, t14337, t14340, t14343, t14345, t14352, t14364, t14372, t14373, t14374, t14379, t14380, t9394);
        let t14612 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1500::<F>(t10577, t10582, t10584, t10586, t14385, t14388, t14392, t14396, t14428, t14433, t14434, t9514, t9517, t9521, t9524);
        let (t14615, t14618, t14620, t14621, t14622) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1501::<F>(t1531, t37, t2612, t4392, t72, t757, t14425, t150, t190, t10608, t2258, t4402);
    (t14608, t14609, t14610, t14612, t14615, t14618, t14620, t14621, t14622)
}
