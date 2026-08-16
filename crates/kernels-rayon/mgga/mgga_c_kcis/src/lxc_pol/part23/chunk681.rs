//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 681/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk681(t1490: f64, t7931: f64, t303: f64, t1498: f64, t553: f64, t2237: f64, t2239: f64, t7895: f64, t7898: f64, t7901: f64, t7906: f64, t7908: f64, t7911: f64, t7916: f64, t7922: f64, t7926: f64, t7929: f64) -> (f64, f64, f64, f64, f64) {
    let t7932 = t7931 * t1490;
    let t7933 = t303 * t7932;
    let t7935 = t553 * t1498;
    let t7936 = t303 * t7935;
    let t7938 = -0.69505208333333333333e-3_f64 * t7895 * t2239 + 0.92754700520833333333e-4_f64 * t7898 * t7901 - t7906 - 0.23168402777777777778e-3_f64 * t7908 * t7911 + 0.69505208333333333333e-3_f64 * t2237 * t7916 + 0.69505208333333333333e-3_f64 * t2237 * t7901 + t7922 + 0.16581944444444444444e-2_f64 * t7926 + 0.24872916666666666666e-2_f64 * t7929 - 0.24872916666666666666e-2_f64 * t7933 + 0.16581944444444444444e-2_f64 * t7936;
    (t7932, t7933, t7935, t7936, t7938)
}
