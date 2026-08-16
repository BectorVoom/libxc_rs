//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 508/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk508(t1235: f64, t4046: f64, t4007: f64, t344: f64, t4038: f64, t1242: f64, t313: f64, t353: f64, t964: f64, t1163: f64, t1248: f64, t3979: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4047 = t1235 * t4046;
    let t4049 = 0.39862222222222222223e0_f64 * t4007;
    let t4054 = 1.0_f64/f64::sqrt(t344);
    let t4055 = t4054 * t4038;
    let t4057 = t1242 * t4046;
    let t4060 = t353 * t964 * t313;
    let t4061 = 0.27385555555555555555e0_f64 * t4060;
    let t4063 = t1248 * t3979 * t1163;
    (t4047, t4049, t4054, t4055, t4057, t4060, t4061, t4063)
}
