//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta399 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1498;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1499;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1500;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta399(t14605: f64, t2482: f64, t2801: f64, t10443: f64, t10552: f64, t10554: f64, t14312: f64, t14313: f64, t14315: f64, t14317: f64, t14324: f64, t14327: f64, t14329: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t10566: f64, t10568: f64, t14333: f64, t14335: f64, t14337: f64, t14340: f64, t14343: f64, t14345: f64, t14352: f64, t14364: f64, t14372: f64, t14373: f64, t14374: f64, t14379: f64, t14380: f64, t9394: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t14385: f64, t14388: f64, t14392: f64, t14396: f64, t14428: f64, t14433: f64, t14434: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64, t1531: f64, t37: f64, t2612: f64, t4392: f64, t72: f64, t757: f64, t14425: f64, t150: f64, t190: f64, t10608: f64, t2258: f64, t4402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14608, t14609) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1498(t14605, t2482, t2801, t10443, t10552, t10554, t14312, t14313, t14315, t14317, t14324, t14327, t14329, t9278, t9308, t9316, t9329, t9333);
        let t14610 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1499(t10566, t10568, t14333, t14335, t14337, t14340, t14343, t14345, t14352, t14364, t14372, t14373, t14374, t14379, t14380, t9394);
        let t14612 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1500(t10577, t10582, t10584, t10586, t14385, t14388, t14392, t14396, t14428, t14433, t14434, t9514, t9517, t9521, t9524);
        let (t14615, t14618, t14620, t14621, t14622) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1501(t1531, t37, t2612, t4392, t72, t757, t14425, t150, t190, t10608, t2258, t4402);
    (t14608, t14609, t14610, t14612, t14615, t14618, t14620, t14621, t14622)
}
