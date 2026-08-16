//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1163/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1163(t7583: f64, t92241: f64, t26597: f64, t26611: f64, t92256: f64, t92258: f64, t92260: f64, t92263: f64, t92266: f64, t92268: f64, t92271: f64, t92273: f64, t92276: f64, t92278: f64, t92280: f64, t92282: f64, t92284: f64, t92286: f64) -> f64 {
    let t92288 = t92241 * t7583;
    let t92290 = t26597 * t26611;
    let t92292 = -0.8347923046875e-3_f64 * t92256 - 0.41703125000000000001e-2_f64 * t92258 + 0.12985658072916666667e-2_f64 * t92260 - 0.16217881944444444444e-1_f64 * t92263 + 0.48653645833333333332e-2_f64 * t92266 - 0.48653645833333333332e-2_f64 * t92268 + 0.208515625e-2_f64 * t92271 + 0.208515625e-2_f64 * t92273 + 0.2782641015625e-3_f64 * t92276 - 0.41703125000000000001e-2_f64 * t92278 + 0.208515625e-2_f64 * t92280 + 0.97307291666666666666e-2_f64 * t92282 - 0.83479230468750000001e-3_f64 * t92284 + 0.2782641015625e-3_f64 * t92286 - 0.97307291666666666666e-2_f64 * t92288 - 0.48653645833333333332e-2_f64 * t92290;
    t92292
}
