//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 869/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk869(t16976: f64, t17038: f64, t17098: f64, t17144: f64, t17194: f64, t17373: f64, t17425: f64, t17504: f64, t1023: f64, t1058: f64, t149: f64, t165: f64, t16659: f64, t16661: f64, t16664: f64, t16932: f64, t3313: f64, t3414: f64, t3588: f64, t4650: f64, t4720: f64, t4837: f64, t564: f64, t614: f64) -> f64 {
    let t17507 = t16976 + t17038 + t17098 + t17144 + t17194 + t17373 + t17425 + t17504;
    let t17509 = -2.0_f64 * t1023 * t3588 - 2.0_f64 * t1058 * t3313 - 2.0_f64 * t1058 * t3414 - t149 * t17507 - t165 * t16659 - t165 * t16661 - t165 * t16664 - t165 * t16932 - t4650 * t614 - t4720 * t614 - t4837 * t564;
    t17509
}
