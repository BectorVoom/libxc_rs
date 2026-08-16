//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1781;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1782;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta407(t18263: f64, t707: f64, t10605: f64, t6002: f64, t2411: f64, t6079: f64, t10446: f64, t5819: f64, t2375: f64, t5825: f64, t13309: f64, t13310: f64, t30: f64, t33: f64, zeta_threshold: f64, t45: f64, t57: f64, t4186: f64, t4377: f64, t606: f64, t78: f64, t10457: f64, t2382: f64, t4384: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18265, t18267, t18268, t18272, t18277, t18280) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1781(t18263, t707, t10605, t6002, t2411, t6079, t10446, t5819, t2375, t5825, t13309, t13310);
        let t18281 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1782(t30, t33, t18280, zeta_threshold);
        let (t18285, t18286, t18297) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1783(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
    (t18265, t18267, t18268, t18272, t18280, t18281, t18285, t18286, t18297)
}
