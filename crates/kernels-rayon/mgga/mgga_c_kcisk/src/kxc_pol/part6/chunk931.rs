//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 931/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk931(t1935: f64, t29528: f64, t17821: f64, t9051: f64, t28950: f64, t747: f64, t746: f64, t1948: f64, t7320: f64, t8972: f64, t29494: f64, t29496: f64, t29499: f64, t29501: f64, t29505: f64, t29507: f64, t29514: f64, t29517: f64, t29520: f64, t29524: f64, t29526: f64) -> (f64, f64, f64, f64, f64) {
    let t29529 = t1935 * t29528;
    let t29531 = t17821 * t9051;
    let t29533 = t747 * t28950;
    let t29534 = t746 * t29533;
    let t29535 = t1948 * t29534;
    let t29537 = t7320 * t8972;
    let t29539 = t29494 / 16.0_f64 + t29496 / 6.0_f64 - t29499 / 3.0_f64 - t29501 / 4.0_f64 - t29505 / 16.0_f64 + 11.0_f64 / 6.0_f64 * t29507 + 209.0_f64 / 216.0_f64 * t29514 + t29517 / 4.0_f64 + t29520 / 36.0_f64 + t29524 / 864.0_f64 - t29526 / 24.0_f64 - 11.0_f64 / 6.0_f64 * t29529 - 3.0_f64 / 128.0_f64 * t29531 + t29535 / 256.0_f64 + 3.0_f64 / 256.0_f64 * t29537;
    (t29529, t29531, t29535, t29537, t29539)
}
