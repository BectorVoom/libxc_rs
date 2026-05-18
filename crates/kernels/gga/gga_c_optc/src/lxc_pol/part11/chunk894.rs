//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk894<F: Float>(t16708: F, t818: F, t16861: F, t799: F, t2416: F, t16784: F, t837: F, t10409: F, t10419: F, t10485: F, t16785: F, t16817: F, t16820: F, t16824: F, t16826: F, t16828: F, t16860: F, t16864: F, t16866: F, t16869: F, t2518: F, t2537: F, t3754: F, t4869: F, t4904: F, t4920: F, t4923: F, t7753: F, t7813: F, t829: F) -> (F, F, F, F, F) {
    let t16872 = t16708 * t818;
    let t16875 = t16861 * t799;
    let t16877 = F::new(6.0) * t2416 * t16875;
    let t16880 = t16784 * t837;
    let t16883 = F::new(0.17544670192365612213e1) * t3754 * t4920 + F::new(0.51947267698127589899e2) * t10409 * t4923 - F::new(0.1038945353962551798e3) * t7813 * t16785 + F::new(0.58482233974552040708e0) * t829 * t16817 + F::new(0.1025389702100779493e4) * t7753 * t16820 - t16824 - t16826 - t16828 - t16860 - t16864 + t16866 + t16869 - F::new(6.0) * t10419 * t4869 + F::new(6.0) * t2518 * t16872 - t16877 - F::new(0.35089340384731224426e1) * t10485 * t4904 + F::new(0.35089340384731224426e1) * t2537 * t16880;
    (t16872, t16875, t16877, t16880, t16883)
}
