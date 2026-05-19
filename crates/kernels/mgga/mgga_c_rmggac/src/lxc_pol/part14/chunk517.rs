//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 517/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk517<F: Float>(t1503: F, t5694: F, t31: F, t4518: F, t1466: F, t4522: F, t605: F, t1182: F, t221: F, t1468: F, t1184: F, t5572: F) -> (F, F, F, F, F, F, F) {
    let t5696 = F::cast_from(0.12805126321218922714e0_f64) * t5694 * t1503;
    let t5697 = t4518 * t31;
    let t5698 = t5697 * t1466;
    let t5699 = t605 * t4522;
    let t5700 = t5699 * t1182;
    let t5701 = t221 * t5700;
    let t5704 = t1468 * t1182;
    let t5705 = t221 * t5704;
    let t5709 = t221 * t5572 * t1184;
    (t5696, t5698, t5700, t5701, t5704, t5705, t5709)
}
