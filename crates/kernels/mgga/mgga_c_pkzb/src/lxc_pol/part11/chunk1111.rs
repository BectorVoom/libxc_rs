//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1111/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1111<F: Float>(t29753: F, t30193: F, t30195: F, t30197: F, t30200: F, t30203: F, t30205: F, t30208: F, t30211: F, t30213: F, t30216: F, t30219: F, t30221: F, t30223: F, t30225: F, t30227: F, t30230: F, t30234: F, t30236: F, t30238: F) -> (F,) {
    let t30239 = -t29753 - t30193 + t30195 - t30197 + t30200 + t30203 - t30205 - t30208 - t30211 + t30213 + t30216 + t30219 - t30221 + t30223 - t30225 + t30227 - t30230 - t30234 + t30236 + t30238;
    (t30239,)
}
