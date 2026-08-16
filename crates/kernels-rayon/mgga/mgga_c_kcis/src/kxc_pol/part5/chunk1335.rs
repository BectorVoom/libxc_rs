//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1335/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1335(t16025: f64, t5489: f64, t1098: f64, t7246: f64, t7234: f64, t1102: f64, t11384: f64, t11632: f64, t16401: f64, t16408: f64, t16410: f64, t16436: f64, t16441: f64, t16543: f64, t21453: f64, t22044: f64, t22049: f64, t22055: f64, t22060: f64, t22064: f64, t22067: f64, t22069: f64, t22072: f64, t22076: f64, t4587: f64, t486: f64) -> f64 {
    let t22079 = t16025 * t5489;
    let t22082 = t1098 * t7246;
    let t22085 = t1098 * t7234;
    let t22089 = 0.10950716111111111111e-2_f64 * t1102 * t22044 + 0.492782225e-3_f64 * t1102 * t22049 + 0.43802864444444444443e-3_f64 * t16401 + 0.7391733375e-3_f64 * t1102 * t22055 - 0.1478346675e-2_f64 * t1102 * t22060 - 0.295669335e-2_f64 * t1102 * t22064 - 0.87605728888888888887e-3_f64 * t22067 - t16408 + t16410 + 0.13140859333333333333e-2_f64 * t22069 + 0.19711289e-2_f64 * t1102 * t22072 - 0.39422578e-2_f64 * t4587 * t22076 - 0.19711289e-2_f64 * t11632 * t22079 + 0.13140859333333333333e-2_f64 * t22082 + t16436 - 0.65704296666666666667e-3_f64 * t16441 + 0.492782225e-3_f64 * t22085 - 4.0_f64 * t486 * t21453 + t11384 + t16543;
    t22089
}
