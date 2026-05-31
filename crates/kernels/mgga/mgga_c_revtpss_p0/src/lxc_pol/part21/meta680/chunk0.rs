//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2493/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2493<F: Float>(t3362: F, t414: F, t12269: F, t1261: F, t12884: F, t247: F, t13085: F, t3647: F, t12277: F, t3634: F, t13089: F, t12273: F) -> (F, F, F, F, F, F) {
    let t44361 = F::cast_from(1.0_f64) / t414 / t3362;
    let t44403 = t1261 * t247 * t12884 * t12269;
    let t44405 = t3647 * t13085;
    let t44409 = t1261 * t247 * t3634 * t12277;
    let t44411 = t3647 * t13089;
    let t44415 = t1261 * t247 * t3634 * t12273;
    (t44361, t44403, t44405, t44409, t44411, t44415)
}
