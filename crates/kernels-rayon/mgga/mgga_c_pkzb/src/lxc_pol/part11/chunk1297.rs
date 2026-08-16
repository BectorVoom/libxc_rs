//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1297/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1297(t22230: f64, t22290: f64, t22800: f64, t22811: f64, t22812: f64, t27358: f64, t27361: f64, t27370: f64, t27373: f64, t31240: f64, t31242: f64, t31250: f64, t31254: f64, t31258: f64, t31262: f64, t31265: f64, t31268: f64, t31271: f64, t31273: f64) -> f64 {
    let t31575 = -0.48204333333333333333e1_f64 * t22230 + t22800 - 0.27785333333333333333e1_f64 * t22290 + t22811 + t22812 + 0.3529725e1_f64 * t31240 + 0.6311625e0_f64 * t31242 + 0.104195e1_f64 * t27358 - 0.125034e1_f64 * t27361 - 0.62517e0_f64 * t27370 - 0.62517e0_f64 * t27373 + 0.937755e0_f64 * t31250 + 0.937755e0_f64 * t31254 + 0.312585e0_f64 * t31258 + 0.312585e0_f64 * t31262 - 0.62517e0_f64 * t31265 - 0.20839e0_f64 * t31268 + 0.794188125e1_f64 * t31271 - 0.473371875e0_f64 * t31273;
    t31575
}
