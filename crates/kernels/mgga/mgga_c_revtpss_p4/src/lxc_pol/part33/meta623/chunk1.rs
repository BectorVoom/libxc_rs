//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2063/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2063<F: Float>(t93317: F, t98852: F, t2439: F, t7774: F, t93170: F, t25304: F, t27212: F, t25301: F, t93371: F, t27286: F, t689: F, t25431: F) -> (F, F, F, F, F, F) {
    let t98856 = F::cast_from(0.15421710918628844644e0_f64) * t93317 * t98852;
    let t98857 = t7774 * t2439;
    let t98858 = t93170 * t98857;
    let t98867 = t25304 * t27212;
    let t98868 = t98867 * t25301;
    let t98875 = t93371 * t98857;
    let t98877 = t27286 * t689;
    let t98879 = F::cast_from(0.14456046980341999104e-1_f64) * t25431 * t98877;
    (t98856, t98858, t98868, t98875, t98877, t98879)
}
