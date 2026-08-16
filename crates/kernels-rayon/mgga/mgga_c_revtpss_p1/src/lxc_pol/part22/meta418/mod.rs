//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2024;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2025;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta418(t13312: f64, t190: f64, t706: f64, t4391: f64, t705: f64, t707: f64, t189: f64, t4186: f64, t606: f64, t4401: f64, t10579: f64, t2411: f64, t4537: f64, t10446: f64, t1469: f64, t2375: f64, t45: f64, t2251: f64, t2258: f64, t4377: f64, t78: f64, t10457: f64, t2382: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14383, t14385, t14386) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2024(t13312, t190, t706, t4391, t705);
        let (t14388, t14390, t14392, t14396, t14397, t14401, t14404) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2025(t14386, t707, t189, t4186, t606, t4401, t10579, t2411, t4537, t10446, t1469, t2375);
        let (t14412, t14413, t14416) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2026(t45, t13312, t14401, t14404, t2251, t2258, t4377, t606, t78, t10457, t1469, t2382, t4186, zeta_threshold);
    (t14383, t14385, t14386, t14388, t14390, t14392, t14396, t14397, t14401, t14412, t14413, t14416)
}
