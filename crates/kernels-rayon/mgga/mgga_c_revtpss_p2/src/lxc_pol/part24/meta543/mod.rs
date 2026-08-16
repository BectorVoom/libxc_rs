//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta543(t45: f64, t57: f64, t18272: f64, t22671: f64, t2375: f64, t39825: f64, t4377: f64, t5825: f64, t78: f64, t87107: f64, t87126: f64, t87145: f64, t18286: f64, t2382: f64, t39840: f64, t4384: f64, t81: f64, zeta_threshold: f64, t162: f64, t187: f64, t150: f64, t190: f64, t18850: f64, t2403: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t39442: f64, t5962: f64, t87262: f64, t87263: f64, t87265: f64, t87267: f64, t87268: f64, t61090: f64, t76947: f64, t76949: f64, t76951: f64, t49897: f64, t18259: f64, t23216: f64, t1469: f64, t4401: f64, t77042: f64, t18263: f64, t5999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87280, t87292) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603(t45, t57, t18272, t22671, t2375, t39825, t4377, t5825, t78, t87107, t87126, t87145, t18286, t2382, t39840, t4384, t81, zeta_threshold);
        let (t87296, t87298, t87302) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604(t87280, t87292, t162, t187, t150, t190, t18850, t2403, t39419, t39422, t39429, t39432, t39442, t5962, t87262, t87263, t87265, t87267, t87268);
        let (t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605(t61090, t76947, t76949, t76951, t49897, t18259, t23216, t1469, t4401, t77042, t18263, t5999);
    (t87296, t87298, t87302, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314)
}
