//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3324/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3324(t15003: f64, t51258: f64, t18784: f64, t2465: f64, t686: f64, t72: f64, t4481: f64, t51276: f64, t6042: f64, t786: f64, t867: f64, t2467: f64) -> (f64, f64, f64, f64) {
    let t63058 = t51258 * t15003;
    let t63062 = t2465 * t18784 * t72 * t686;
    let t63064 = t51276 * t4481;
    let t63084 = t786 * t6042 * t867;
    let t63085 = t63084 * t2467;
    (t63058, t63062, t63064, t63085)
}
