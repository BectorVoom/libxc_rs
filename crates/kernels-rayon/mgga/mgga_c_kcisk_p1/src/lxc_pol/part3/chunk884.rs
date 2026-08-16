//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 884/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk884(t1175: f64, t12970: f64, t12992: f64, t13244: f64, t13247: f64, t13250: f64, t13253: f64, t13274: f64, t1355: f64, t306: f64, t3559: f64, t3587: f64, t3599: f64, t3602: f64) -> f64 {
    let t13277 = 3.0_f64 / 16.0_f64 * t13244 * t12970 - 3.0_f64 / 8.0_f64 * t13247 * t3559 - 3.0_f64 / 8.0_f64 * t3599 * t13250 + 3.0_f64 / 4.0_f64 * t13253 * t1175 + 3.0_f64 / 4.0_f64 * t3602 * t3587 + t1355 * t12992 / 4.0_f64 + t306 * t13274 / 2.0_f64;
    t13277
}
