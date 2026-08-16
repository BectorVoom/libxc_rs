//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1866/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1866(t3601: f64, t3783: f64, t7660: f64, t1269: f64, t3140: f64, t1276: f64, t2148: f64, t1243: f64, t8939: f64) -> (f64, f64, f64) {
    let t26913 = t7660 * t3601 * t3783;
    let t26916 = t1269 * t3140;
    let t26918 = t2148 * t26916 * t1276;
    let t26921 = t8939 * t1243;
    (t26913, t26918, t26921)
}
