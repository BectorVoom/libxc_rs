//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1200/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1200<F: Float>(t1037: F, t1080: F, t1082: F, t1095: F, t21874: F, t21982: F, t21994: F, t22033: F, t22038: F, t22042: F, t22045: F, t22050: F, t22068: F, t22072: F, t22076: F, t22094: F, t221: F, t2639: F, t2771: F, t2774: F, t2783: F, t2788: F, t2789: F, t2791: F, t2809: F, t492: F, t7359: F, t7410: F) -> F {
    let t22276 = F::cast_from(0.69263436422725855036e2_f64) * t2809 * t7410 * t2639 * t1095 + F::cast_from(36.0_f64) * t2789 * t2774 * t2783 + t21982 + t21994 + t22033 + t22038 + t22042 + t22045 - t22050 + F::cast_from(0.12865583598954028054e3_f64) * t2789 * t7359 * t2791 * t1080 - F::cast_from(0.18989649058080861537e-2_f64) * t221 * t21874 * t492 - t22068 + t22072 + F::cast_from(0.41096e0_f64) * t1037 * t2771 * t2783 * t1082 - F::cast_from(0.6609050294782684211e1_f64) * t1037 * t2788 * t2783 * t2791 * t1080 - t22076 - t22094;
    t22276
}
