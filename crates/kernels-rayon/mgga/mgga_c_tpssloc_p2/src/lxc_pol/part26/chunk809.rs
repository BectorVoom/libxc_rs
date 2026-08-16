//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 809/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk809(t52: f64, t607: f64, t78: f64, t2250: f64, t638: f64, t771: f64, t9258: f64, t9288: f64, t9505: f64, zeta_threshold: f64) -> f64 {
    let t150 = t52 <= zeta_threshold;
    let t9508 = t78 * t607;
    let t9514 = piecewise3(t150, 0.0_f64, -8.0_f64 / 27.0_f64 * t638 * t9288 - 2.0_f64 / 3.0_f64 * t9508 * t2250 - 2.0_f64 / 3.0_f64 * t771 * t9258);
    let t9516 = t9505 / 2.0_f64 + t9514 / 2.0_f64;
    t9516
}
