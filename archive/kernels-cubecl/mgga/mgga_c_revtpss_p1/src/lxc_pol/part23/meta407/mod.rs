//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1781;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1782;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1783;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta407<F: Float>(t18263: F, t707: F, t10605: F, t6002: F, t2411: F, t6079: F, t10446: F, t5819: F, t2375: F, t5825: F, t13309: F, t13310: F, t30: F, t33: F, zeta_threshold: F, t45: F, t57: F, t4186: F, t4377: F, t606: F, t78: F, t10457: F, t2382: F, t4384: F, t81: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18265, t18267, t18268, t18272, t18277, t18280) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1781::<F>(t18263, t707, t10605, t6002, t2411, t6079, t10446, t5819, t2375, t5825, t13309, t13310);
        let t18281 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1782::<F>(t30, t33, t18280, zeta_threshold);
        let (t18285, t18286, t18297) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1783::<F>(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
    (t18265, t18267, t18268, t18272, t18280, t18281, t18285, t18286, t18297)
}
