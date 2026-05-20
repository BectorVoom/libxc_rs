//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2067;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2068;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta437<F: Float>(t14494: F, t2749: F, t14791: F, t775: F, t836: F, t14586: F, t10693: F, t10706: F, t10711: F, t10713: F, t10717: F, t10719: F, t10723: F, t10730: F, t10734: F, t10742: F, t14788: F, t2745: F, t4362: F, t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t10905: F, t4442: F, t4457: F, t800: F, t1548: F, t2430: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14792, t14793, t14802) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2067::<F>(t14494, t2749, t14791, t775, t836);
        let (t14803, t14804, t14811) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2068::<F>(t14586, t14802, t14791, t10693, t10706, t10711, t10713, t10717, t10719, t10723, t10730, t10734, t10742, t14788, t14793, t2745, t4362);
        let (t14817, t14819, t14820, t14823, t14825, t14829) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2069::<F>(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t4457, t775, t800, t1548, t2430);
    (t14792, t14793, t14802, t14803, t14804, t14811, t14817, t14819, t14820, t14823, t14825, t14829)
}
