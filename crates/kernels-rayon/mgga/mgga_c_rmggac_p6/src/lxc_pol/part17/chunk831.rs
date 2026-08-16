//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 831/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk831(t40965: f64, t7835: f64, t39666: f64, t7788: f64, t262: f64, t40805: f64, t7782: f64, t1587: f64, t664: f64, t2067: f64, t26: f64, t25525: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40966 = t7835 * t40965;
    let t40967 = 0.36366215538993788972e-1_f64 * t40966;
    let t40970 = t7788 * t39666;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40983 = t664 * t1587;
    let t40998 = t2067 * t26;
    let t40999 = t25525 * t40998;
    (t40967, t40970, t40975, t40976, t40983, t40998, t40999)
}
