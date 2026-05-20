//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta543 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta543<F: Float>(t45: F, t57: F, t18272: F, t22671: F, t2375: F, t39825: F, t4377: F, t5825: F, t78: F, t87107: F, t87126: F, t87145: F, t18286: F, t2382: F, t39840: F, t4384: F, t81: F, zeta_threshold: F, t162: F, t187: F, t150: F, t190: F, t18850: F, t2403: F, t39419: F, t39422: F, t39429: F, t39432: F, t39442: F, t5962: F, t87262: F, t87263: F, t87265: F, t87267: F, t87268: F, t61090: F, t76947: F, t76949: F, t76951: F, t49897: F, t18259: F, t23216: F, t1469: F, t4401: F, t77042: F, t18263: F, t5999: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87280, t87292) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1603::<F>(t45, t57, t18272, t22671, t2375, t39825, t4377, t5825, t78, t87107, t87126, t87145, t18286, t2382, t39840, t4384, t81, zeta_threshold);
        let (t87296, t87298, t87302) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1604::<F>(t87280, t87292, t162, t187, t150, t190, t18850, t2403, t39419, t39422, t39429, t39432, t39442, t5962, t87262, t87263, t87265, t87267, t87268);
        let (t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1605::<F>(t61090, t76947, t76949, t76951, t49897, t18259, t23216, t1469, t4401, t77042, t18263, t5999);
    (t87296, t87298, t87302, t87303, t87304, t87305, t87306, t87307, t87309, t87312, t87314)
}
