//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 942/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk942(t29653: f64, t29667: f64, t1974: f64, t12061: f64, t29636: f64, t12002: f64, t15989: f64, t22564: f64, t22575: f64, t22583: f64, t28371: f64, t28375: f64, t28379: f64, t28383: f64, t28387: f64, t28391: f64) -> (f64, f64, f64) {
    let t29668 = t29653 + t29667;
    let t29669 = t29668 * t1974;
    let t29672 = t29636 * t12061;
    let t29685 = -t12002 - 0.2283111111111111111e-1_f64 * t15989 + 0.11415555555555555555e-1_f64 * t22564 - 0.34246666666666666665e-1_f64 * t22575 + 0.17123333333333333333e-1_f64 * t22583 - 0.19025925925925925925e-1_f64 * t28371 + 0.68493333333333333331e-1_f64 * t28375 - 0.34246666666666666665e-1_f64 * t28379 - 0.10274e0_f64 * t28383 + 0.10274e0_f64 * t28387 - 0.17123333333333333333e-1_f64 * t28391;
    (t29669, t29672, t29685)
}
