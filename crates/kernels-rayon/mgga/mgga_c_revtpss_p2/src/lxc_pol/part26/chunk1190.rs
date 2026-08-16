//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1190/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1190(t2471: f64, t26563: f64, t10985: f64, t26576: f64, t2062: f64, t2769: f64, t786: f64, t10997: f64, t26519: f64, t93157: f64, t2453: f64, t2458: f64, t7399: f64) -> (f64, f64, f64, f64, f64) {
    let t95927 = t26563 * t2471;
    let t95930 = 0.46263278077393568556e-2_f64 * t26576 * t10985;
    let t95936 = t786 * t2062 * t2769;
    let t95937 = t95936 * t10997;
    let t95945 = t93157 * t26519;
    let t95948 = t2453 * t7399 * t2458;
    (t95927, t95930, t95937, t95945, t95948)
}
