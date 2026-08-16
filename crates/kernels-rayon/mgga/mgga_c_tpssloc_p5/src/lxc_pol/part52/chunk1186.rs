//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1186/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1186(t5: f64, t8662: f64, t9231: f64, t9239: f64, t131: f64, t7245: f64, t2240: f64, t7254: f64, t8301: f64, t31019: f64, t31677: f64, t31684: f64, t31693: f64, t8515: f64, t8663: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t31857 = t9231 * t8662;
    let t31860 = t9239 * t8662;
    let t31863 = t7245 * t131;
    let t31864 = t2240 * t31863;
    let t31867 = t8301 * t7254;
    let t31868 = t2240 * t31867;
    let t31876 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t31857 * t8515 - 5.0_f64 / 24.0_f64 * t31860 * t31677 - 5.0_f64 / 36.0_f64 * t31864 * t31684 + 5.0_f64 / 144.0_f64 * t31868 * t8515 + 5.0_f64 / 72.0_f64 * t8663 * t31693 + 5.0_f64 / 144.0_f64 * t8663 * t31019);
    (t31857, t31860, t31863, t31864, t31867, t31868, t31876)
}
