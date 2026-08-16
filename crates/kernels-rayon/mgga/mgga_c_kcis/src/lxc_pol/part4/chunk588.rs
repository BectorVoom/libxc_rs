//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 588/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk588(t3073: f64, t3074: f64, t2943: f64, t308: f64, t1042: f64, t932: f64, t2917: f64, t2919: f64, t2922: f64, t2925: f64, t2928: f64, t2945: f64, t2953: f64) -> (f64, f64, f64, f64, f64) {
    let t3075 = t3073 * t3074;
    let t3078 = t2943 * t308;
    let t3081 = t932 * t1042;
    let t3088 = 0.55033333333333333333e-2_f64 * t2917;
    let t3093 = -0.991e-2_f64 * t2945 + 0.1982e-1_f64 * t2953 + t3088 + 0.27516666666666666666e-2_f64 * t2919 - 0.27516666666666666667e-2_f64 * t2922 + 0.8255e-2_f64 * t2925 - 0.41275e-2_f64 * t2928;
    (t3075, t3078, t3081, t3088, t3093)
}
