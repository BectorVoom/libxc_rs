//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1191/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1191(t2039: f64, t22461: f64, t26103: f64, t31237: f64, t31239: f64, t31532: f64, t31700: f64, t31704: f64, t31706: f64, t31708: f64, t31716: f64, t31719: f64, t31721: f64, t6517: f64, t671: f64, t7056: f64, t8446: f64) -> f64 {
    let t31722 = 2.0_f64 * t2039 * t22461 + 2.0_f64 * t2039 * t26103 + 2.0_f64 * t31532 * t671 + 2.0_f64 * t6517 * t7056 + t31237 + t31239 + t31700 + t31704 + t31706 + t31708 + t31716 + t31719 + t31721 + t8446;
    t31722
}
