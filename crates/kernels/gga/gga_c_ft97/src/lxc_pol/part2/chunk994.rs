//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 994/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk994<F: Float>(t312: F, t9577: F, t13863: F, t4139: F, t18: F, t824: F, t2875: F, t2874: F, t4311: F, t840: F, t1882: F, t4252: F) -> (F, F, F, F) {
    let t15402 = t312 * t9577;
    let t15403 = t15402 * t13863;
    let t15404 = t4139 * t15403;
    let t15407 = t18 * t824;
    let t15408 = t2875 * t15407;
    let t15409 = t2874 * t15408;
    let t15415 = t840 * t4311 * t824;
    let t15419 = F::new(2.0) / F::new(9.0) * t1882 * t4252;
    (t15404, t15409, t15415, t15419)
}
