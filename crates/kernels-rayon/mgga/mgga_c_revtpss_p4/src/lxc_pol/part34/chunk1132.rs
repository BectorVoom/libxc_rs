//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1132/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1132(t27418: f64, t994: f64, t27638: f64, t3143: f64, t1983: f64, t1647: f64, t1976: f64, t3336: f64, t7840: f64, t33: f64, t892: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27661 = t994 * t27418;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27699 = t1647 * t1976;
    let t27712 = t7840 * t3336;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    (t27661, t27668, t27669, t27699, t27712, t27763, t27799)
}
