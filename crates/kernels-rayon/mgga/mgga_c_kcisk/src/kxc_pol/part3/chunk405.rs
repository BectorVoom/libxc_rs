//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 405/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk405(t71: f64, t20: f64, t79: f64, t2863: f64, t2866: f64, t873: f64, t80: f64, t2958: f64, t2960: f64, t2962: f64, t880: f64, t861: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2964 = 1.0_f64/f64::sqrt(t71);
    let t2966 = t2964 * t79 * t20;
    let t2967 = t2966 * t2863;
    let t2969 = t873 * t2866;
    let t2971 = t80 * t2863;
    let t2973 = -0.42198333333333333333e0_f64 * t2958 + 0.84396666666666666666e0_f64 * t2960 + 0.39862222222222222223e0_f64 * t2962 + 0.68258333333333333333e-1_f64 * t2967 + 0.13651666666666666667e0_f64 * t2969 + 0.13692777777777777778e0_f64 * t2971;
    let t2974 = t2973 * t880;
    let t2977 = t861 * t861;
    (t2966, t2967, t2969, t2971, t2973, t2974, t2977)
}
