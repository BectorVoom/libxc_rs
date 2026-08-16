//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1009/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1009(t1041: f64, t10413: f64, t14117: f64, t14160: f64, t14203: f64, t1618: f64, t17885: f64, t17907: f64, t18005: f64, t18008: f64, t18030: f64, t21532: f64, t21538: f64, t21542: f64, t21546: f64, t21551: f64, t973: f64) -> f64 {
    let t21560 = -t10413 * t21532 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t17885 - t14117 / 4608.0_f64 - t973 * t21538 / 36.0_f64 + t973 * t21542 / 288.0_f64 + 7.0_f64 / 648.0_f64 * t973 * t21546 - t17907 / 1152.0_f64 - t1041 * t21551 / 768.0_f64 + t18030 * t1618 / 1024.0_f64 - t14160 / 432.0_f64 + t18005 / 1536.0_f64 + t18008 / 1152.0_f64 - t14203 / 6912.0_f64;
    t21560
}
