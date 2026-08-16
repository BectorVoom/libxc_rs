//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1095/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1095(t6539: f64, t7709: f64, t5329: f64, t1768: f64, t27924: f64, t303: f64, t2173: f64, t2175: f64, t26781: f64, t26837: f64, t28928: f64, t28932: f64, t28936: f64, t28939: f64, t28945: f64, t28948: f64, t28952: f64, t7703: f64) -> (f64, f64, f64, f64, f64) {
    let t28957 = t7709 * t6539;
    let t28958 = t5329 * t28957;
    let t28961 = t27924 * t1768;
    let t28962 = t303 * t28961;
    let t28964 = -0.69505208333333333333e-3_f64 * t28932 * t2175 + 0.22109259259259259258e-2_f64 * t28936 + 0.46336805555555555556e-3_f64 * t7703 * t28939 + 0.46336805555555555556e-3_f64 * t7703 * t28928 + 0.16581944444444444444e-2_f64 * t28945 + 0.69505208333333333333e-3_f64 * t2173 * t28948 - 0.13901041666666666667e-2_f64 * t2173 * t28952 - t26837 - 0.185671721767578125e-4_f64 * t26781 * t28952 + 0.69505208333333333333e-3_f64 * t2173 * t28958 - 0.49745833333333333332e-2_f64 * t28962;
    (t28957, t28958, t28961, t28962, t28964)
}
