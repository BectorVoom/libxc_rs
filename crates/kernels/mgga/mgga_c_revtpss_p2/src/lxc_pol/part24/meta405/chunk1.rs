//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1344/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1344<F: Float>(t2452: F, t9720: F, t225: F, t268: F, t10868: F, t240: F, t2237: F, t2482: F, t849: F, t234: F, t9801: F, t136: F, t2475: F) -> (F, F, F, F, F, F, F) {
    let t40688 = t9720 * t2452;
    let t40689 = t40688 * t225;
    let t40690 = t268 * t40689;
    let t40693 = t10868 * t240;
    let t40710 = t2482 * t849 * t2237;
    let t40721 = t9801 * t234;
    let t40724 = t2475 * t136;
    (t40688, t40689, t40690, t40693, t40710, t40721, t40724)
}
