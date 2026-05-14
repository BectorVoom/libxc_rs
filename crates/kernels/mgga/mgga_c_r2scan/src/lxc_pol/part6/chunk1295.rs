//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1295/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1295<F: Float>(t51: F, t2520: F, t409: F, t1216: F, t1224: F, t19363: F, t23869: F, t23872: F, t23878: F, t23881: F, t2517: F, t35: F, t40: F, t419: F, t476: F, t4920: F, t4921: F, t4927: F, t7073: F, t7076: F, t893: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t24405 = 16.0 * t2520 * t409;
    let t24407 = piecewise3(t52, 0.0, -56.0 / 81.0 * t19363 * t893 * t4921 - 16.0 / 9.0 * t4920 * t35 * t23869 + 8.0 / 9.0 * t7073 * t23872 + 4.0 / 3.0 * t1224 * t1216 * t419 - 4.0 * t7076 * t23878 + 4.0 / 3.0 * t7076 * t23881 - 2.0 / 9.0 * t2517 * t4927 + 8.0 * t476 * t40 - t24405);
    (t24407,)
}
