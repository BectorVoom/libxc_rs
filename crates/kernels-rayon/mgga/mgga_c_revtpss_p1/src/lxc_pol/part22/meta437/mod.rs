//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2068;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta437(t14494: f64, t2749: f64, t14791: f64, t775: f64, t836: f64, t14586: f64, t10693: f64, t10706: f64, t10711: f64, t10713: f64, t10717: f64, t10719: f64, t10723: f64, t10730: f64, t10734: f64, t10742: f64, t14788: f64, t2745: f64, t4362: f64, t2710: f64, t2713: f64, t4371: f64, t4353: f64, t808: f64, t10744: f64, t10905: f64, t4442: f64, t4457: f64, t800: f64, t1548: f64, t2430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14792, t14793, t14802) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2067(t14494, t2749, t14791, t775, t836);
        let (t14803, t14804, t14811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2068(t14586, t14802, t14791, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t14788, t14793, t2745, t4362);
        let (t14817, t14819, t14820, t14823, t14825, t14829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2069(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t4457, t775, t800, t1548, t2430);
    (t14792, t14793, t14802, t14803, t14804, t14811, t14817, t14819, t14820, t14823, t14825, t14829)
}
