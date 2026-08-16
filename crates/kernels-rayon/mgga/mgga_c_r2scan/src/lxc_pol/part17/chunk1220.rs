//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1220/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1220(t38452: f64, t39429: f64, t39464: f64, t39470: f64, t39482: f64, t41384: f64, t41385: f64, t41386: f64, t41387: f64, t41392: f64, t43057: f64, t43061: f64) -> f64 {
    let t44209 = 0.10975748638225852664e0_f64 * t43057 + 0.62295486109113302474e-1_f64 * t39429 + t41384 - t41385 - t41386 + t41387 - t41392 - 0.23804984598836975487e0_f64 * t39464 - 0.57829097596741960691e-3_f64 * t39470 + 0.87327386630866483588e-2_f64 * t43061 - t38452 + 0.62295486109113302474e-1_f64 * t39482;
    t44209
}
