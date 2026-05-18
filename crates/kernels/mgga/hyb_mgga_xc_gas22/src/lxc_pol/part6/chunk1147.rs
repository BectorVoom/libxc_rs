//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1147/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1147<F: Float>(t513: F, t5198: F, t1118: F, t4535: F, t1123: F, t4524: F, t1145: F, t1129: F, t11289: F, t11293: F, t1130: F, t11411: F, t11479: F, t11482: F, t11485: F, t2834: F, t2838: F, t3713: F, t3714: F, t3717: F, t3753: F, t3788: F, t4565: F, t4571: F, t4577: F, t7734: F, t7780: F, t7806: F, t9558: F, t9782: F) -> (F, F, F, F, F, F, F) {
    let t11512 = t5198 * t513;
    let t11515 = t1118 * t4535;
    let t11520 = t4524 * t1123;
    let t11521 = t1145 * t11520;
    let t11524 = t4524 * t1129;
    let t11525 = t1145 * t11524;
    let t11530 = F::new(32.0) * t7806 * t11479 - F::new(112.0) / F::new(3.0) * t9558 * t11482 + F::new(352.0) / F::new(243.0) * t3753 * t11485 - F::new(128.0) / F::new(81.0) * t3753 * t11411 - F::new(4.0) * t4571 * t1130 + F::new(2.0) * t3788 * t4565 + F::new(88.0) / F::new(9.0) * t2834 * t11289 - F::new(88.0) / F::new(9.0) * t2838 * t11293 - F::new(200.0) / F::new(9.0) * t11512 * t3714 + F::new(800.0) / F::new(27.0) * t3713 * t11515 + F::new(800.0) / F::new(27.0) * t3717 * t11515 + F::new(126.0) * t7734 * t11521 - F::new(168.0) * t7780 * t11525 - F::new(4.0) * t9782 * t4577;
    (t11512, t11515, t11520, t11521, t11524, t11525, t11530)
}
