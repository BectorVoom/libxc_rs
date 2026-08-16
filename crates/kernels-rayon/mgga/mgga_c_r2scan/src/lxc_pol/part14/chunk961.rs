//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 961/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk961(t10998: f64, t11189: f64, t3275: f64, t1149: f64, t1353: f64, t10940: f64, t3469: f64, t10626: f64, t3465: f64, t3492: f64, t498: f64) -> (f64, f64, f64, f64, f64) {
    let t11191 = t3275 * t11189 * t10998;
    let t11192 = 45.0_f64 / 64.0_f64 * t11191;
    let t11193 = t1353 * t1149;
    let t11194 = t10940 * t3469;
    let t11195 = t11194 / 4.0_f64;
    let t11197 = t3275 * t3465 * t10626;
    let t11198 = t11197 / 2.0_f64;
    let t11199 = t498 * t3492;
    (t11192, t11193, t11195, t11198, t11199)
}
