//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1146/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1146<F: Float>(t30213: F, t30216: F, t30219: F, t30221: F, t30223: F, t30225: F, t30227: F, t30230: F, t30234: F, t30236: F, t30238: F, t30242: F, t30245: F, t30248: F, t30252: F, t30255: F, t30259: F, t30261: F, t30263: F, t30265: F, t30268: F, t30270: F) -> (F, F) {
    let t30991 = t30213 + t30216 + t30219 - t30221 + t30223 - t30225 + t30227 - t30230 - t30234 + t30236 + t30238;
    let t30993 = -t30242 - t30245 - t30248 + t30252 + t30255 + t30259 + t30261 - t30263 - t30265 - t30268 - t30270;
    (t30991, t30993)
}
