//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2412;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2413;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta568<F: Float>(t30: F, t33: F, t18280: F, zeta_threshold: F, t45: F, t57: F, t18272: F, t18277: F, t4186: F, t4377: F, t606: F, t78: F, t10457: F, t5819: F, t2382: F, t5825: F, t4384: F, t81: F, t150: F, t190: F, t5944: F, t750: F, t189: F, t4401: F, t10552: F, t10554: F, t14317: F, t18253: F, t18256: F, t18261: F, t18262: F, t18265: F, t18267: F, t18268: F, t1940: F, t2403: F, t4537: F, t4541: F, t4556: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t18281 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2412::<F>(t30, t33, t18280, zeta_threshold);
        let (t18285, t18286, t18291, t18297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2413::<F>(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
        let (t18298, t18299, t18300, t18301, t18305, t18306, t18308, t18309) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2414::<F>(t18285, t18297, t150, t190, t5944, t750, t189, t5825, t606, t4401, t10552, t10554, t14317, t18253, t18256, t18261, t18262, t18265, t18267, t18268, t1940, t2403, t4537, t4541, t4556, t775, t9278, t9308, t9316, t9329, t9333);
    (t18281, t18286, t18291, t18298, t18299, t18300, t18301, t18305, t18306, t18308, t18309)
}
