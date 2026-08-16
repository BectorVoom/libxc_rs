//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1176/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1176(t1949: f64, t2718: f64, t198: f64, t1993: f64, t11064: f64, t30: f64, t33: f64, t892: f64, t1955: f64, t7283: f64, t13846: f64, t1941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27357 = t2718 * t1949;
    let t27382 = t198 * t1993;
    let t27383 = t11064 * t30;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    let t27932 = t1941 * t13846;
    (t27357, t27382, t27383, t27763, t27799, t27868, t27932)
}
