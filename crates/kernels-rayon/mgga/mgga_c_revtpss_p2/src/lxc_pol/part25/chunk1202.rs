//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1202/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1202(t1096: f64, t4975: f64, t27638: f64, t3143: f64, t1983: f64, t33: f64, t892: f64, t11064: f64, t1955: f64, t7283: f64, t13846: f64, t1941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27664 = t4975 * t1096;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    let t27932 = t1941 * t13846;
    (t27664, t27668, t27669, t27763, t27799, t27868, t27932)
}
