//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 844/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk844(t33: f64, t892: f64, t4433: f64, t18875: f64, t25759: f64, t1113: f64, t1544: f64, t4343: f64, t27375: f64, t11064: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27763 = t892 * t33;
    let t27764 = t27763 * t4433;
    let t27770 = t25759 * t18875;
    let t27773 = t1113 * t1544;
    let t27777 = t33 * t4343;
    let t27793 = t25759 * t27375;
    let t27799 = t11064 * t33;
    (t27764, t27770, t27773, t27777, t27793, t27799)
}
