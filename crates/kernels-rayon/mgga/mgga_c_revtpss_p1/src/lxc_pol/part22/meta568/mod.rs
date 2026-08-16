//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2412;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2413;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta568(t30: f64, t33: f64, t18280: f64, zeta_threshold: f64, t45: f64, t57: f64, t18272: f64, t18277: f64, t4186: f64, t4377: f64, t606: f64, t78: f64, t10457: f64, t5819: f64, t2382: f64, t5825: f64, t4384: f64, t81: f64, t150: f64, t190: f64, t5944: f64, t750: f64, t189: f64, t4401: f64, t10552: f64, t10554: f64, t14317: f64, t18253: f64, t18256: f64, t18261: f64, t18262: f64, t18265: f64, t18267: f64, t18268: f64, t1940: f64, t2403: f64, t4537: f64, t4541: f64, t4556: f64, t775: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t18281 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2412(t30, t33, t18280, zeta_threshold);
        let (t18285, t18286, t18291, t18297) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2413(t45, t57, t18272, t18277, t18281, t4186, t4377, t606, t78, t10457, t5819, t2382, t5825, t4384, t81, zeta_threshold);
        let (t18298, t18299, t18300, t18301, t18305, t18306, t18308, t18309) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2414(t18285, t18297, t150, t190, t5944, t750, t189, t5825, t606, t4401, t10552, t10554, t14317, t18253, t18256, t18261, t18262, t18265, t18267, t18268, t1940, t2403, t4537, t4541, t4556, t775, t9278, t9308, t9316, t9329, t9333);
    (t18281, t18286, t18291, t18298, t18299, t18300, t18301, t18305, t18306, t18308, t18309)
}
