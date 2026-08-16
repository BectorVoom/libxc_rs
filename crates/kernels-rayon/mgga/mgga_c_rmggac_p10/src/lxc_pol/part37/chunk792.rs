//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 792/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk792(t74283: f64, t1971: f64, t2144: f64, t3351: f64, t41006: f64, t68422: f64, t68440: f64, t9122: f64, t2367: f64, t352: f64, t875: f64, t14025: f64, t21713: f64, t40167: f64, t9212: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74284 = 0.24829349937757072983e-4_f64 * t74283;
    let t74287 = t3351 * t1971 * t2144 * t41006;
    let t74290 = t68440 * t68422 * t9122;
    let t74292 = t2367 * t352;
    let t74295 = t3351 * t1971 * t875 * t74292;
    let t74299 = t21713 * t14025 * t40167 * t9212;
    (t74284, t74287, t74290, t74292, t74295, t74299)
}
