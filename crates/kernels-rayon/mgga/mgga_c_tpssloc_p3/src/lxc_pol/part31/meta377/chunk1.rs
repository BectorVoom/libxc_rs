//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1329/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1329(t13042: f64, t13053: f64, t13065: f64, t13463: f64, t1528: f64, t17083: f64, t17087: f64, t17090: f64, t17092: f64, t17095: f64, t17098: f64, t17100: f64, t259: f64, t2597: f64, t4268: f64, t4273: f64, t5658: f64, t866: f64) -> f64 {
    let t17108 = -2.0_f64 * t13042 * t1528 - 2.0_f64 * t13053 * t1528 - 2.0_f64 * t13065 * t1528 - 2.0_f64 * t13463 * t1528 + t17083 * t259 + 2.0_f64 * t17087 * t259 - t17090 * t866 - 2.0_f64 * t17092 * t866 + 2.0_f64 * t17095 * t259 + t17098 * t259 + t17100 * t259 - t2597 * t5658 + 4.0_f64 * t4268 * t4273;
    t17108
}
