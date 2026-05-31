//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1020/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1020<F: Float>(t7573: F, t9503: F, t2824: F, t3727: F, t3739: F, t3747: F, t3753: F, t3757: F, t7806: F, t9508: F, t9513: F, t9521: F, t9523: F, t9527: F, t9528: F, t9533: F, t9535: F, t9538: F, t9542: F, t9545: F, t9549: F, t9552: F, t9558: F, sigma2: F) -> (F, F, F) {
    let t9561 = t7573 * sigma2;
    let t9562 = t9561 * t9503;
    let t9565 = F::cast_from(176.0_f64) / F::cast_from(81.0_f64) * t3747 * t9513 + F::cast_from(352.0_f64) / F::cast_from(243.0_f64) * t3753 * t9508 + F::cast_from(176.0_f64) / F::cast_from(81.0_f64) * t3757 * t9513 - F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t9521 * t9523 - F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t9527 * t9528 + F::cast_from(100.0_f64) / F::cast_from(3.0_f64) * t9533 * t9535 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t9538 * t3727 * t2824 - F::cast_from(500.0_f64) / F::cast_from(3.0_f64) * t9542 * t9523 + F::cast_from(32.0_f64) * t7806 * t9545 + F::cast_from(200.0_f64) * t9549 * t9535 + F::cast_from(32.0_f64) * t7806 * t9552 - F::cast_from(200.0_f64) * t9549 * t9523 - F::cast_from(112.0_f64) / F::cast_from(3.0_f64) * t9558 * t9528 - F::cast_from(128.0_f64) / F::cast_from(81.0_f64) * t3739 * t9562;
    (t9561, t9562, t9565)
}
