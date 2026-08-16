//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 210/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk210(t4: f64, t668: f64, t11: f64, t19: f64, t662: f64, t210: f64, t665: f64, t21: f64, t351: f64, t5: f64) -> (f64, f64, f64, f64, f64) {
    let t669 = t4 * t668;
    let t671 = 1.0_f64/f64::sqrt(t11);
    let t672 = t671 * t19;
    let t673 = t672 * t662;
    let t675 = t210 * t665;
    let t678 = t21 * t5 * t351;
    (t669, t672, t673, t675, t678)
}
