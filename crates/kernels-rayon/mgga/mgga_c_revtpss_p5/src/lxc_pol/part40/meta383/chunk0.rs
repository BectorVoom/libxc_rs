//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1380/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1380(t1280: f64, t16750: f64, t3153: f64, t5284: f64, t5465: f64, t1287: f64, t1811: f64, t3588: f64, t13133: f64, t1774: f64, t1214: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16751 = t1280 * t16750;
    let t16756 = t5284 * t3153;
    let t16757 = t16756 * t5465;
    let t16763 = t1811 * t3588 * t1287;
    let t16768 = t13133 * t1774;
    let t16771 = t5245 * t1214;
    (t16751, t16756, t16757, t16763, t16768, t16771)
}
