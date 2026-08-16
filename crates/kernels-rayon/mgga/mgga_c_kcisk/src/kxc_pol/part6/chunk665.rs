//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 665/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk665(t9240: f64, t9257: f64, t2666: f64, t8973: f64, t9017: f64, t9021: f64, t9023: f64, t9025: f64, t9027: f64, t9031: f64, t9033: f64, t9037: f64, t9039: f64, t9041: f64, t9044: f64) -> (f64, f64, f64) {
    let t9258 = t9240 + t9257;
    let t9262 = t2666 * t2666;
    let t9277 = 0.101171875e-1_f64 * t8973 + 0.9375e-1_f64 * t9017 - 0.20833333333333333333e-1_f64 * t9021 + 0.20234375e-1_f64 * t9023 - 0.5e0_f64 * t9025 + 0.125e0_f64 * t9027 - 0.9375e-1_f64 * t9031 - 0.1875e0_f64 * t9033 + 0.625e-1_f64 * t9037 + 0.10791666666666666667e0_f64 * t9039 - 0.26979166666666666666e-1_f64 * t9041 + 0.5e0_f64 * t9044;
    (t9258, t9262, t9277)
}
