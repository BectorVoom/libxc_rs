//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1970/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1970(t1774: f64, t8197: f64, t7637: f64, t2148: f64, t6695: f64, t1287: f64, t6622: f64, t7660: f64, t26907: f64, t3769: f64, t6628: f64, t1769: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30866 = t8197 * t1774;
    let t30867 = t7637 * t30866;
    let t30870 = t2148 * t6695;
    let t30874 = t7660 * t6622 * t1287;
    let t30878 = t26907 * t6628 * t3769;
    let t30881 = t1769 * t1769;
    (t30866, t30867, t30870, t30874, t30878, t30881)
}
