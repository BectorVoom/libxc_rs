//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 690/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk690<F: Float>(t1550: F, t9951: F, t1756: F, t2060: F, t739: F, t515: F, t6522: F, t3352: F, t3351: F, t2286: F, t8571: F, t558: F, t615: F) -> (F, F, F, F, F, F, F) {
    let t9952 = t1550 * t9951;
    let t9957 = t2060 * t1756;
    let t9958 = t739 * t9957;
    let t9963 = t515 * t6522;
    let t9964 = t3352 * t9963;
    let t9965 = t3351 * t9964;
    let t9967 = t8571 * t2286;
    let t9969 = t558 * t615;
    (t9952, t9957, t9958, t9964, t9965, t9967, t9969)
}
