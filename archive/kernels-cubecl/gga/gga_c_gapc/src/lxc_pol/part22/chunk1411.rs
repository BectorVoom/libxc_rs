//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1411/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1411<F: Float>(t35031: F, t35036: F, t35039: F, t35045: F, t35048: F, t35051: F, t35056: F, t35059: F, t35062: F, t35034: F, t35041: F, t35069: F) -> (F, F) {
    let t37172 = F::cast_from(0.40483072916666666669e-4_f64) * t35031;
    let t37174 = F::cast_from(0.50680539737635041234e-3_f64) * t35036;
    let t37175 = F::cast_from(0.25301920572916666668e-5_f64) * t35039;
    let t37177 = F::cast_from(0.38673709012042260328e-8_f64) * t35045;
    let t37178 = F::cast_from(0.80966145833333333338e-4_f64) * t35048;
    let t37179 = F::cast_from(0.984817913114256917e-7_f64) * t35051;
    let t37180 = F::cast_from(0.15716995342493974597e-7_f64) * t35056;
    let t37181 = F::cast_from(0.42206481990611010728e-7_f64) * t35059;
    let t37182 = F::cast_from(0.99044544404633838508e-5_f64) * t35062;
    let t37183 = -t37172 - F::cast_from(0.44198524585191154658e-7_f64) * t35034 - t37174 - t37175 - F::cast_from(0.57970906942607043474e-5_f64) * t35041 - t37177 - t37178 - t37179 + t37180 + t37181 - t37182;
    let t37184 = F::cast_from(0.26519114751114692796e-6_f64) * t35069;
    (t37183, t37184)
}
