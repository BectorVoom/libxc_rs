//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1320/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1320(t33661: f64, t28739: f64, t28743: f64, t33624: f64, t33626: f64, t33630: f64, t33633: f64, t33637: f64, t33640: f64, t33642: f64, t33645: f64, t33649: f64, t33651: f64, t33653: f64, t33656: f64, t33659: f64) -> f64 {
    let t33662 = 0.85206502119823888168e-1_f64 * t33661;
    let t33663 = -t33624 - t33626 - t33630 - t28739 - t28743 + t33633 - t33637 - t33640 - t33642 - t33645 - t33649 - t33651 - t33653 + t33656 + t33659 - t33662;
    t33663
}
