//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 358/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk358<F: Float>(t143: F, t1246: F, t1249: F, t1251: F, t1253: F, t1255: F, t1257: F, t1259: F, t1261: F, t1264: F, t1279: F, t172: F, t187: F, t693: F) -> (F,) {
    let t144 = 0.135e1 <= t143;
    let t1283 = piecewise3(t144, -t693 * t1246 / 18.0 + t1249 / 240.0 - t1251 / 4480.0 + t1253 / 103680.0 - t1255 / 2838528.0 + t1257 / 89456640.0 - t1259 / 0.31850496e10 + t1261 / 0.1263403008e12, -8.0 / 3.0 * t1264 * t187 - 8.0 / 3.0 * t172 * t1279);
    (t1283,)
}
