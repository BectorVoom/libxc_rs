//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 988/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk988(t3555: f64, t3754: f64, t1248: f64, t3153: f64, t3588: f64, t5464: f64, t3566: f64, t3568: f64, t1287: f64, t1269: f64, t1284: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12713 = t5464 * t3588;
    let t12714 = t12712 * t12713;
    let t12717 = t3566 * t3754;
    let t12718 = t3568 * t1248;
    let t12719 = t12718 * t1287;
    let t12722 = t1284 * t1269;
    let t12723 = t1209 * t12722;
    (t12709, t12714, t12717, t12718, t12719, t12723)
}
