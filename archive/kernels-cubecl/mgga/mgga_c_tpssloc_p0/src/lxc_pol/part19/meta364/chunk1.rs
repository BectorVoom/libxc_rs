//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1327/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1327<F: Float>(t10523: F, t41827: F, t951: F, t959: F, t300: F, t41764: F, t10853: F, t2940: F, t2925: F, t2951: F, t2929: F, t2932: F, t41733: F) -> (F, F, F, F, F) {
    let t42697 = F::cast_from(0.14035736694323150897e2_f64) * t959 * t10523 * t41827 * t951;
    let t42699 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t41764;
    let t42701 = F::cast_from(0.20779030926817756511e3_f64) * t2940 * t10853;
    let t42704 = F::cast_from(0.21053605041484726346e2_f64) * t959 * t2951 * t2925;
    let t42708 = F::cast_from(0.51947577317044391277e2_f64) * t959 * t2929 * t41733 * t2932;
    (t42697, t42699, t42701, t42704, t42708)
}
