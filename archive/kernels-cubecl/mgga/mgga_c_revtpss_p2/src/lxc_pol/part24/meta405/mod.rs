//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1343;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1344;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta405<F: Float>(t16: F, t2236: F, t240: F, t236: F, t243: F, t281: F, t39644: F, t10696: F, t72: F, t245: F, t10697: F, t136: F, t2452: F, t9720: F, t225: F, t268: F, t10868: F, t2237: F, t2482: F, t849: F, t234: F, t9801: F, t2475: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40649, t40650, t40654, t40673, t40683) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1343::<F>(t16, t2236, t240, t236, t243, t281, t39644, t10696, t72, t245, t10697, t136);
        let (t40688, t40689, t40690, t40693, t40710, t40721, t40724) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1344::<F>(t2452, t9720, t225, t268, t10868, t240, t2237, t2482, t849, t234, t9801, t136, t2475);
    (t40649, t40650, t40654, t40673, t40683, t40688, t40689, t40690, t40693, t40710, t40721, t40724)
}
