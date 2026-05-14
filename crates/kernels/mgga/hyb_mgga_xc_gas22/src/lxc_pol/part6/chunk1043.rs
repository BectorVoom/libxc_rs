//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1043/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1043<F: Float>(t4143: F, t809: F, t6579: F, t4140: F, t2188: F, t2236: F, t4139: F, t2234: F, t3352: F, t3356: F, t4113: F, t6564: F, t6562: F, t3419: F, t3435: F, t4180: F, t6640: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10645 = t4143 * t809;
    let t10647 = 0.96491876992155210402e2 * t6579 * t10645;
    let t10648 = t4140 * t809;
    let t10650 = 2.0 * t2188 * t10648;
    let t10651 = t4139 * t2236;
    let t10652 = t10651 * t809;
    let t10654 = 0.16081979498692535067e2 * t2234 * t10652;
    let t10655 = t3356 * t3352;
    let t10657 = 0.32163958997385070134e2 * t2234 * t10655;
    let t10658 = t4113 * t6564;
    let t10659 = t10658 * t809;
    let t10661 = 0.51726012919273400301e3 * t6562 * t10659;
    let t10662 = t3435 * t3419;
    let t10667 = t6640 * t4180;
    (t10645, t10647, t10648, t10650, t10651, t10652, t10654, t10655, t10657, t10658, t10659, t10661, t10662, t10667)
}
