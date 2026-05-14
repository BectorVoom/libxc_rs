//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1018/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1018<F: Float>(t13805: F, t1580: F, t1445: F, t47026: F, t597: F, t46945: F, t40449: F, t40452: F, t40455: F, t40458: F, t13810: F, t4950: F, t42315: F, t42316: F, t42350: F, t42354: F) -> (F,) {
    let t48131 = t1580 * t13805;
    let t48134 = t597 * t1445 * t47026;
    let t48137 = t597 * t1445 * t46945;
    let t48140 = 0.63904876589867916128e-1 * t40449;
    let t48141 = 0.31952438294933958064e0 * t40452;
    let t48142 = 0.51123901271894332903e0 * t40455;
    let t48143 = 0.38342925953920749677e0 * t40458;
    let t48144 = t4950 * t13810;
    let t48146 = 0.11502877786176224903e2 * t48131 + 0.11502877786176224903e2 * t48134 + 0.11502877786176224903e2 * t48137 - t42315 - 0.14896037479937677779e-1 * t42316 + t48140 + t48141 - t48142 + t48143 - t42350 + 0.71500979903700853338e0 * t48144 + t42354;
    (t48146,)
}
