//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 858/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk858<F: Float>(t312: F, t9577: F, t13863: F, t4139: F, t18: F, t824: F, t2875: F, t2874: F, t4311: F, t840: F, t1882: F, t4252: F, t1225: F, t8232: F, t15074: F, t296: F) -> (F, F, F, F, F, F) {
    let t15402 = t312 * t9577;
    let t15403 = t15402 * t13863;
    let t15404 = t4139 * t15403;
    let t15407 = t18 * t824;
    let t15408 = t2875 * t15407;
    let t15409 = t2874 * t15408;
    let t15415 = t840 * t4311 * t824;
    let t15419 = 2.0 / 9.0 * t1882 * t4252;
    let t15420 = t8232 * t1225;
    let t15422 = t296 * t15074;
    (t15404, t15409, t15415, t15419, t15420, t15422)
}
