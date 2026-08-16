//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1161/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1161(t1276: f64, t2148: f64, t26916: f64, t1243: f64, t8939: f64, t2149: f64, t1248: f64, t1287: f64, t7653: f64, t1294: f64, t7638: f64, t7652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26918 = t2148 * t26916 * t1276;
    let t26921 = t8939 * t1243;
    let t26922 = t2149 * t26921;
    let t26924 = t7653 * t1248 * t1287;
    let t26927 = t7638 * t1294;
    let t26928 = t7652 * t26927;
    (t26918, t26921, t26922, t26924, t26927, t26928)
}
