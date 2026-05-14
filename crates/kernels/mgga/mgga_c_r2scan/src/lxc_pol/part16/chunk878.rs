//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 878/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk878<F: Float>(t1053: F, t1102: F, t11572: F, t10653: F, t10657: F, t11478: F, t11482: F, t11485: F, t11489: F, t11492: F, t11495: F, t11500: F, t11504: F, t11508: F, t11512: F, t11566: F, t11570: F) -> (F,) {
    let t11574 = t1102 * t1053 * t11572;
    let t11577 = -0.15243824895787514157e-3 * t11566 + 0.21684485328539747656e-4 * t11570 + t11478 + t11482 - t11485 + t11489 - t11492 + 0.15243824895787514157e-3 * t11574 + t11495 + t11500 - t11504 - t11508 - t11512 + 0.36021158228745895953e-3 * t10653 - t10657;
    (t11577,)
}
