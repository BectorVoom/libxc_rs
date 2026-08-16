//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 576/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk576(t1106: f64, t3255: f64, t1098: f64, t1111: f64, t1116: f64, t2840: f64, t346: f64, t2844: f64, t347: f64, t1018: f64, t245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3256 = t3255 * t1106;
    let t3258 = t1098 * t1111;
    let t3260 = t1098 * t1116;
    let t3262 = t2840 * t346;
    let t3263 = t347 * t2844;
    let t3268 = t1018 * t245;
    let t3269 = t3268 * t347;
    (t3256, t3258, t3260, t3262, t3263, t3269)
}
