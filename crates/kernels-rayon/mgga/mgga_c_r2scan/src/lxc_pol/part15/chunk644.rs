//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 644/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk644(t1058: f64, t2207: f64, t3606: f64, t2608: f64, t3332: f64, t2147: f64, t269: f64, t978: f64) -> (f64, f64, f64, f64) {
    let t3608 = t2207 * t1058 * t3606;
    let t3610 = t3332 * t2608;
    let t3611 = t2147 * t3610;
    let t3613 = t978 * t269;
    (t3608, t3610, t3611, t3613)
}
