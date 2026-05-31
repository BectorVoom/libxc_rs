//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1432/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1432<F: Float>(t3740: F, t3951: F, t9645: F, t9656: F, t11474: F, t11478: F, t9567: F, t11410: F, t3957: F, t22746: F, t22750: F, t2940: F, t30689: F, t30692: F, t30793: F, t3753: F, t3757: F, t4550: F, t9521: F, t9642: F, t9654: F, t9667: F, t9678: F, sigma2: F) -> (F, F, F, F) {
    let t31039 = t3740 * t3951;
    let t31040 = t31039 * t9645;
    let t31043 = t31039 * t9656;
    let t31046 = t11474 * sigma2;
    let t31050 = t11478 * sigma2;
    let t31051 = t31050 * t9567;
    let t31054 = t11410 * t3957;
    let t31055 = t31054 * t9645;
    let t31058 = t31054 * t9656;
    let t31065 = F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t9521 * t30793 + F::cast_from(1408.0_f64) / F::cast_from(243.0_f64) * t3753 * t30692 + F::cast_from(704.0_f64) / F::cast_from(81.0_f64) * t3757 * t30689 + F::cast_from(12.0_f64) * t2940 * t4550 + F::cast_from(1408.0_f64) / F::cast_from(243.0_f64) * t9678 * t31040 - F::cast_from(1408.0_f64) / F::cast_from(243.0_f64) * t9667 * t31043 + F::cast_from(320.0_f64) * t22746 * t31046 * t9567 - F::cast_from(448.0_f64) * t22750 * t31051 - F::cast_from(512.0_f64) / F::cast_from(27.0_f64) * t9642 * t31055 + F::cast_from(512.0_f64) / F::cast_from(27.0_f64) * t9654 * t31058 + F::cast_from(1408.0_f64) / F::cast_from(81.0_f64) * t9642 * t31040 - F::cast_from(1408.0_f64) / F::cast_from(81.0_f64) * t9654 * t31043;
    (t31051, t31055, t31058, t31065)
}
