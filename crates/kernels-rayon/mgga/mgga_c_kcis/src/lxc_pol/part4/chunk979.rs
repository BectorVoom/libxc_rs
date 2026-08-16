//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 979/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk979(t3066: f64, t331: f64, t10112: f64, t313: f64, t1031: f64, t1068: f64, t1046: f64, t3054: f64, t3069: f64, t1027: f64, t3097: f64, t308: f64, t9758: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10184 = t331 * t3066;
    let t10187 = 0.14055920378328537299e-1_f64 * t10112 * t313;
    let t10188 = t1068 * t1031;
    let t10190 = t3054 * t1046;
    let t10192 = t331 * t3069;
    let t10194 = t1027 * t3097;
    let t10199 = t9758 * t308;
    (t10184, t10187, t10188, t10190, t10192, t10194, t10199)
}
