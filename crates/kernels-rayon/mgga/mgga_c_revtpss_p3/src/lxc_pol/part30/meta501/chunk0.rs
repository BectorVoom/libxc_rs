//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1868/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1868(t1248: f64, t1287: f64, t7653: f64, t1294: f64, t7638: f64, t7652: f64, t1243: f64, t7627: f64, t1032: f64, t1269: f64) -> (f64, f64, f64, f64, f64) {
    let t26924 = t7653 * t1248 * t1287;
    let t26927 = t7638 * t1294;
    let t26928 = t7652 * t26927;
    let t26931 = t1243 * t7627;
    let t26933 = t26931 * t1248 * t1287;
    let t26936 = t1269 * t1032;
    (t26924, t26928, t26931, t26933, t26936)
}
