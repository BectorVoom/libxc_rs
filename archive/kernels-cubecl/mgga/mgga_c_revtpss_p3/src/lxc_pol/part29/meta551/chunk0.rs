//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1888/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1888<F: Float>(t7289: F, t96370: F, t7284: F, t96282: F, t94669: F, t96271: F, t26277: F, t94913: F, t25944: F, t96259: F, t1385: F, t7506: F) -> (F, F, F, F, F, F) {
    let t96371 = t7289 * t96370;
    let t96374 = F::cast_from(0.22487184191643109717e-1_f64) * t7284 * t96282;
    let t96378 = t94669 * t96271;
    let t96380 = t94913 * t26277;
    let t96382 = t25944 * t96259;
    let t96392 = t1385 * t7506;
    (t96371, t96374, t96378, t96380, t96382, t96392)
}
