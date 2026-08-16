//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1435/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1435(t28738: f64, t28742: f64, t33616: f64, t33619: f64, t33624: f64, t33626: f64, t33630: f64, t33633: f64, t33637: f64, t33640: f64, t33642: f64, t33645: f64, t33649: f64, t33651: f64, t33653: f64, t33656: f64) -> f64 {
    let t39249 = t33616 - t33619 - t33624 - t33626 - t33630 - 0.15337170381568299871e1_f64 * t28738 - 0.15337170381568299871e1_f64 * t28742 + t33633 - t33637 - t33640 - t33642 - t33645 - t33649 - t33651 - t33653 + t33656;
    t39249
}
