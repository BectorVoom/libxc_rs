//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 696/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk696<F: Float>(t884: F, t9954: F, t1756: F, t2060: F, t739: F, t2024: F, t1356: F, t515: F, t6522: F, t3352: F, t3351: F, t2286: F, t8571: F) -> (F, F, F, F, F, F, F, F) {
    let t9955 = t884 * t9954;
    let t9956 = F::cast_from(0.23948483403727617128e0_f64) * t9955;
    let t9957 = t2060 * t1756;
    let t9958 = t739 * t9957;
    let t9959 = F::cast_from(0.14967802127329760705e-1_f64) * t9958;
    let t9960 = t2024 * t1756;
    let t9961 = t1356 * t9960;
    let t9962 = F::cast_from(0.39914139006212695214e-1_f64) * t9961;
    let t9963 = t515 * t6522;
    let t9964 = t3352 * t9963;
    let t9965 = t3351 * t9964;
    let t9966 = F::cast_from(0.25538759935978703638e-4_f64) * t9965;
    let t9967 = t8571 * t2286;
    (t9956, t9957, t9959, t9960, t9962, t9964, t9966, t9967)
}
