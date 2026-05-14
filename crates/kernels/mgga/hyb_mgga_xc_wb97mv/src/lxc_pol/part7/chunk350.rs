//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 350/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk350<F: Float>(t43: F, t1187: F, t1190: F, t1192: F, t1194: F, t1196: F, t1198: F, t1200: F, t1202: F, t1205: F, t1220: F, t564: F, t72: F, t88: F) -> (F,) {
    let t44 = 0.135e1 <= t43;
    let t1224 = piecewise3(t44, -t564 * t1187 / 18.0 + t1190 / 240.0 - t1192 / 4480.0 + t1194 / 103680.0 - t1196 / 2838528.0 + t1198 / 89456640.0 - t1200 / 0.31850496e10 + t1202 / 0.1263403008e12, -8.0 / 3.0 * t1205 * t88 - 8.0 / 3.0 * t72 * t1220);
    (t1224,)
}
