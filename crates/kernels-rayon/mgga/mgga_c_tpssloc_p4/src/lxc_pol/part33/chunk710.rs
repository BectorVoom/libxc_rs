//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 710/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk710(t1905: f64, t794: f64, t6562: f64, t6604: f64, t814: f64) -> (f64, f64, f64) {
    let t6643 = t794 * t1905;
    let t6644 = t6562 * t6643;
    let t6645 = 0.41123351671205660912e-2_f64 * t6644;
    let t6646 = t6604 * t814;
    (t6643, t6645, t6646)
}
