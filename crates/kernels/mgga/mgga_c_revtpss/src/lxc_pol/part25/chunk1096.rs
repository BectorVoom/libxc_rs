//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1096/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1096<F: Float>(t45963: F, t6957: F, t10309: F, t25105: F, t45972: F, t10310: F, t77: F, t84: F, t2248: F, t640: F, t45958: F, t10301: F, t10298: F, t607: F, t2242: F, t2259: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92684 = t45963 * t6957;
    let t92687 = t10309 * t25105;
    let t92690 = t45972 * t6957;
    let t92692 = t77 * t84 * t10310;
    let t92696 = t77 * t640 * t2248;
    let t92699 = t45958 * t6957;
    let t92702 = t10301 * t25105;
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    (t92684, t92687, t92690, t92692, t92696, t92699, t92702, t92709, t92711)
}
