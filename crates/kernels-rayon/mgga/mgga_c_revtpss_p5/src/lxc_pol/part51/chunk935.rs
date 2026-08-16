//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 935/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk935(t1043: f64, t1045: f64, t385: f64, t3117: f64, t1032: f64, t3268: f64, t8507: f64, t994: f64, t31948: f64, t8520: f64) -> (f64, f64, f64, f64, f64) {
    let t31977 = t385 * t1043 * t1045;
    let t31978 = t3117 * t31977;
    let t31981 = t1032 * t3268;
    let t31986 = t994 * t8507;
    let t31991 = t8520 * t31948;
    (t31977, t31978, t31981, t31986, t31991)
}
