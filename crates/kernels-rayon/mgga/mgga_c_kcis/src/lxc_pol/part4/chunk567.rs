//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 567/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk567(t2918: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t261: f64, t926: f64, t930: f64, t951: f64, t257: f64, t929: f64, t244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2930 = t2918 + 0.11872222222222222222e-1_f64 * t2919 - 0.11872222222222222222e-1_f64 * t2922 + 0.35616666666666666666e-1_f64 * t2925 - 0.17808333333333333333e-1_f64 * t2928;
    let t2932 = 0.62182e-1_f64 * t2930 * t261;
    let t2933 = t926 * t930;
    let t2935 = 2.0_f64 * t2933 * t951;
    let t2936 = t929 * t257;
    let t2937 = 1.0_f64 / t2936;
    let t2938 = t244 * t2937;
    (t2930, t2932, t2933, t2935, t2937, t2938)
}
