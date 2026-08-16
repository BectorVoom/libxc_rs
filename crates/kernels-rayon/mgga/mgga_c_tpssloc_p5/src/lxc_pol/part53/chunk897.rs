//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 897/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk897(t112: f64, t32262: f64, t2039: f64, t23938: f64, t26977: f64, t31237: f64, t31239: f64, t32206: f64, t32235: f64, t671: f64, t7042: f64, t7056: f64, t8446: f64, t9012: f64) -> (f64, f64) {
    let t32263 = t32262 * t112;
    let t32278 = 4.0_f64 * t2039 * t23938 + 4.0_f64 * t2039 * t26977 + 2.0_f64 * t32235 * t671 + 4.0_f64 * t7042 * t7056 + 4.0_f64 * t7056 * t9012 + t31237 + t31239 + 2.0_f64 * t32206 + t32263 + t8446;
    (t32263, t32278)
}
