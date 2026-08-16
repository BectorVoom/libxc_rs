//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 875/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk875(t3786: f64, t7237: f64, t1477: f64, t6964: f64, t542: f64, t3814: f64, t7122: f64, t1482: f64, t7141: f64, t1924: f64, t1102: f64, t344: f64, t3743: f64, t486: f64, t5423: f64, t5449: f64, t5486: f64, t7028: f64, t7214: f64, t7218: f64, t7222: f64, t7226: f64, t7230: f64, t7234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7238 = t3786 * t7237;
    let t7241 = t1477 * t6964;
    let t7242 = t542 * t7241;
    let t7245 = t3814 * t7122;
    let t7246 = t542 * t7245;
    let t7249 = t1482 * t7141;
    let t7250 = t542 * t7249;
    let t7253 = t1924 * t1924;
    let t7257 = -t3743 + 0.8760572888888888889e-3_f64 * t5423 + 0.19711289e-2_f64 * t5449 - 0.13140859333333333333e-2_f64 * t5486 + 0.10950716111111111111e-2_f64 * t1102 * t7214 + 0.19711289e-2_f64 * t1102 * t7218 - 0.13140859333333333333e-2_f64 * t1102 * t7222 - 0.13140859333333333333e-2_f64 * t1102 * t7226 + 0.65704296666666666667e-3_f64 * t1102 * t7230 + 0.7391733375e-3_f64 * t344 * t7234 - 0.295669335e-2_f64 * t1102 * t7238 + 0.1478346675e-2_f64 * t344 * t7242 + 0.19711289e-2_f64 * t344 * t7246 - 0.98556445e-3_f64 * t344 * t7250 - 4.0_f64 * t7253 - 4.0_f64 * t486 * t7028;
    (t7238, t7241, t7242, t7245, t7246, t7249, t7250, t7257)
}
