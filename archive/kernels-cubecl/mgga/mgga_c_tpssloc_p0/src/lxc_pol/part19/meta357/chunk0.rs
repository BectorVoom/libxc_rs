//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1294/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1294<F: Float>(t2853: F, t2885: F, t10523: F, t938: F, t10660: F, t888: F, t10663: F, t10702: F, t2844: F, t41995: F, t10810: F, t919: F) -> (F, F, F, F, F) {
    let t42123 = t2853 * t2885;
    let t42128 = t938 * t10523;
    let t42143 = t888 * t10660;
    let t42145 = F::cast_from(0.3859675079686208416e3_f64) * t42143 * t10663;
    let t42148 = F::cast_from(0.57895126195293126241e3_f64) * t10702 * t41995 * t2844;
    let t42149 = t919 * t10810;
    (t42123, t42128, t42145, t42148, t42149)
}
