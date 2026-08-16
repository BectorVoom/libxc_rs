//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 696/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk696(t884: f64, t9954: f64, t1756: f64, t2060: f64, t739: f64, t2024: f64, t1356: f64, t515: f64, t6522: f64, t3352: f64, t3351: f64, t2286: f64, t8571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9955 = t884 * t9954;
    let t9956 = 0.23948483403727617128e0_f64 * t9955;
    let t9957 = t2060 * t1756;
    let t9958 = t739 * t9957;
    let t9959 = 0.14967802127329760705e-1_f64 * t9958;
    let t9960 = t2024 * t1756;
    let t9961 = t1356 * t9960;
    let t9962 = 0.39914139006212695214e-1_f64 * t9961;
    let t9963 = t515 * t6522;
    let t9964 = t3352 * t9963;
    let t9965 = t3351 * t9964;
    let t9966 = 0.25538759935978703638e-4_f64 * t9965;
    let t9967 = t8571 * t2286;
    (t9956, t9957, t9959, t9960, t9962, t9964, t9966, t9967)
}
