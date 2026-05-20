//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2024;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2025;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta418<F: Float>(t13312: F, t190: F, t706: F, t4391: F, t705: F, t707: F, t189: F, t4186: F, t606: F, t4401: F, t10579: F, t2411: F, t4537: F, t10446: F, t1469: F, t2375: F, t45: F, t2251: F, t2258: F, t4377: F, t78: F, t10457: F, t2382: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14383, t14385, t14386) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2024::<F>(t13312, t190, t706, t4391, t705);
        let (t14388, t14390, t14392, t14396, t14397, t14401, t14404) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2025::<F>(t14386, t707, t189, t4186, t606, t4401, t10579, t2411, t4537, t10446, t1469, t2375);
        let (t14412, t14413, t14416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2026::<F>(t45, t13312, t14401, t14404, t2251, t2258, t4377, t606, t78, t10457, t1469, t2382, t4186, zeta_threshold);
    (t14383, t14385, t14386, t14388, t14390, t14392, t14396, t14397, t14401, t14412, t14413, t14416)
}
