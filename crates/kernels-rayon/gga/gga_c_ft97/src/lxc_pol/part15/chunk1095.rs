//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1095/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1095(t17630: f64, t4431: f64, t1073: f64, t12116: f64, t12122: f64, t20027: f64, t20035: f64, t2265: f64, t2266: f64, t4462: f64, t48117: f64, t4883: f64, t75994: f64, t76056: f64, t76062: f64, t76101: f64, t76126: f64, t76128: f64, t76130: f64, t8654: f64) -> f64 {
    let t87843 = t17630 * t4431;
    let t87868 = -8.0_f64 * t75994 - 160.0_f64 / 81.0_f64 * t48117 + 8.0_f64 * t2265 * t12116 * t87843 - 4.0_f64 / 3.0_f64 * t2265 * t12122 * t87843 - 16.0_f64 / 3.0_f64 * t76056 + 8.0_f64 / 3.0_f64 * t76062 - 4.0_f64 / 9.0_f64 * t76101 + 8.0_f64 / 9.0_f64 * t76126 + 8.0_f64 / 3.0_f64 * t76128 + 8.0_f64 / 3.0_f64 * t76130 + 8.0_f64 / 3.0_f64 * t2265 * t8654 * t20027 * t1073 - 2.0_f64 * t2265 * t2266 * t4462 * t4883 - 8.0_f64 * t2265 * t2266 * t20035 * t1073;
    t87868
}
