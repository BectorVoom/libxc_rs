//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 757/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk757<F: Float>(t109: F, t4097: F, t1616: F, t417: F, t1470: F, t1291: F, t1308: F, t1295: F, t1300: F, t1304: F, t193: F, t202: F, t210: F, t4077: F, t4083: F, t4087: F, t4088: F, t4090: F, t4094: F) -> (F, F, F, F) {
    let t4098 = t109 * t4097;
    let t4099 = t1616 * t417;
    let t4102 = t1470 * t4097;
    let t4103 = t4102 * t1616;
    let t4106 = t1308 * t1291;
    let t4109 = 0.39111111111111111112e-1 * t193 * t4077 * t202 - 0.38400000000000000001e-3 * t1295 * t4083 * t1300 + 0.91022222222222222228e-6 * t4087 * t4088 * t4090 - 40.0 / 9.0 * t1304 * t4094 + 50.0 / 9.0 * t4098 * t4099 + 50.0 / 9.0 * t210 * t4103 - 40.0 / 9.0 * t210 * t4106;
    (t4099, t4103, t4106, t4109)
}
