//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1245/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1245<F: Float>(t3205: F, t707: F, t1249: F, t1251: F, t1253: F, t1255: F, t1257: F, t1259: F, t1261: F, t154: F, t157: F, t160: F, t30445: F, t30448: F, t711: F, t715: F, t719: F, t723: F, t727: F, t731: F, t735: F, t8756: F) -> (F,) {
    let t30480 = t3205 * t707;
    let t30497 = -t154 * t30448 / 24.0 + t157 * t30448 / 320.0 - t160 * t30448 / 5760.0 + t711 * t30445 / 240.0 - t715 * t30445 / 4480.0 + t719 * t30445 / 103680.0 - t723 * t30445 / 2838528.0 + t727 * t30445 / 89456640.0 - t731 * t30445 / 0.31850496e10 + t735 * t30445 / 0.1263403008e12 + t8756 * t30480 / 103219200.0 - 8.0 / 3.0 * t1249 * t30480 + t1251 * t30480 / 2.0 - t1253 * t30480 / 20.0 + t1255 * t30480 / 288.0 - t1257 * t30480 / 5376.0 + t1259 * t30480 / 122880.0 - t1261 * t30480 / 3317760.0;
    (t30497,)
}
