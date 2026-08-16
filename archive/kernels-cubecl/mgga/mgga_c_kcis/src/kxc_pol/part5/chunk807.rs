//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 807/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk807<F: Float>(t2938: F, t6366: F, t2960: F, t6320: F, t6338: F, t939: F, t2970: F, t6326: F, t26: F, t6330: F, t945: F, t6334: F) -> (F, F, F, F, F, F, F, F) {
    let t6368 = F::cast_from(2.0_f64) * t2938 * t6366;
    let t6375 = t2960 * t6320;
    let t6377 = t939 * t6338;
    let t6380 = t2970 * t6326;
    let t6381 = t26 * t6380;
    let t6383 = t945 * t6330;
    let t6384 = t26 * t6383;
    let t6386 = t945 * t6334;
    (t6368, t6375, t6377, t6380, t6381, t6383, t6384, t6386)
}
