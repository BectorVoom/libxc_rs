//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1092/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1092<F: Float>(t4143: F, t809: F, t6579: F, t4140: F, t2188: F, t2236: F, t4139: F, t2234: F, t3352: F, t3356: F, t4113: F, t6564: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10645 = t4143 * t809;
    let t10647 = F::cast_from(0.96491876992155210402e2_f64) * t6579 * t10645;
    let t10648 = t4140 * t809;
    let t10650 = F::cast_from(2.0_f64) * t2188 * t10648;
    let t10651 = t4139 * t2236;
    let t10652 = t10651 * t809;
    let t10654 = F::cast_from(0.16081979498692535067e2_f64) * t2234 * t10652;
    let t10655 = t3356 * t3352;
    let t10657 = F::cast_from(0.32163958997385070134e2_f64) * t2234 * t10655;
    let t10658 = t4113 * t6564;
    (t10645, t10647, t10648, t10650, t10651, t10652, t10654, t10655, t10657, t10658)
}
