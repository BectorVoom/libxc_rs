//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 492/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk492(t1383: f64, t3579: f64, t1398: f64, t1375: f64, t158: f64, t165: f64, t173: f64, t3278: f64, t3819: f64, t3870: f64, t3873: f64, t3875: f64, t3878: f64, t3881: f64, t3883: f64, t3886: f64, t3891: f64) -> (f64, f64, f64, f64) {
    let t3894 = t1383 * t3579;
    let t3897 = t1398 * t3579;
    let t3900 = t1375 * t3579;
    let t3903 = -0.672175e-5_f64 * t173 * t3870 + 0.9368e-2_f64 * t3873 - 0.3513e-2_f64 * t158 * t3875 + 0.1171e-2_f64 * t158 * t3878 - 0.26416666666666666666e-2_f64 * t3881 + 0.7925e-3_f64 * t165 * t3883 - 0.52833333333333333333e-3_f64 * t165 * t3886 - 0.23911438650126355246e-1_f64 * t3819 * t3278 + 0.15538616723388920628e-3_f64 * t3891 * t3278 - 0.1585e-2_f64 * t165 * t3894 - 0.10082625e-4_f64 * t173 * t3897 + 0.7026e-2_f64 * t158 * t3900;
    (t3894, t3897, t3900, t3903)
}
