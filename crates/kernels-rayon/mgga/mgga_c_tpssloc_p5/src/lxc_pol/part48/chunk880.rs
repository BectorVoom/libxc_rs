//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 880/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk880(t2039: f64, t24932: f64, t27888: f64, t31237: f64, t31239: f64, t31704: f64, t31706: f64, t31708: f64, t31716: f64, t31719: f64, t31721: f64, t32349: f64, t32350: f64, t671: f64, t7056: f64, t7266: f64, t8446: f64) -> f64 {
    let t32359 = 2.0_f64 * t2039 * t24932 + 2.0_f64 * t2039 * t27888 + 2.0_f64 * t32350 * t671 + 2.0_f64 * t7056 * t7266 + t31237 + t31239 + t31704 + t31706 + t31708 + t31716 + t31719 + t31721 + t32349 + t8446;
    t32359
}
