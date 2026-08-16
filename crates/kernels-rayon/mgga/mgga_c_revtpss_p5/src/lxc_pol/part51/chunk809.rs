//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 809/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk809(t25877: f64, t25894: f64, t25881: f64, t1385: f64, t2028: f64, t25875: f64, t1399: f64, t676: f64, t25880: f64, t212: f64, t7274: f64, t1358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25895 = t25894 * t25877;
    let t25896 = t25895 * t25881;
    let t25898 = t2028 * t1385;
    let t25899 = t25875 * t25898;
    let t25900 = t676 * t1399;
    let t25901 = t25880 * t25900;
    let t25902 = t25899 * t25901;
    let t25904 = t25894 * t25898;
    let t25905 = t25904 * t25901;
    let t25912 = t212 * t7274;
    let t25913 = t25912 * t1358;
    (t25895, t25896, t25898, t25899, t25900, t25902, t25904, t25905, t25913)
}
