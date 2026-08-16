//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 837/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk837(t3537: f64, t4612: f64, t6328: f64, t6332: f64, t6336: f64, t1830: f64, t1220: f64, t3557: f64, t3564: f64, t4706: f64, t6341: f64, t6343: f64, t6375: f64, t6377: f64, t6381: f64, t6384: f64, t6387: f64) -> (f64, f64, f64, f64) {
    let t6783 = t3537 + 0.11415555555555555555e-1_f64 * t4612 - 0.11415555555555555555e-1_f64 * t6328 + 0.34246666666666666666e-1_f64 * t6332 - 0.17123333333333333333e-1_f64 * t6336;
    let t6788 = t1830 * t1830;
    let t6789 = t6788 * t1220;
    let t6804 = -0.17648625e1_f64 * t6341 + 0.3529725e1_f64 * t6343 + t3557 + 0.34431666666666666666e0_f64 * t4612 - 0.34431666666666666667e0_f64 * t6328 + 0.103295e1_f64 * t6332 - 0.516475e0_f64 * t6336 + 0.31558125e0_f64 * t6375 + 0.6311625e0_f64 * t6377 + t3564 + 0.13892666666666666667e0_f64 * t4706 - 0.34731666666666666667e-1_f64 * t6381 + 0.20839e0_f64 * t6384 - 0.104195e0_f64 * t6387;
    (t6783, t6788, t6789, t6804)
}
