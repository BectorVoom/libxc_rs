//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 843/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk843(t739: f64, t75141: f64, t14225: f64, t3352: f64, t8436: f64, t1986: f64, t305: f64, t8441: f64, t69619: f64, t8446: f64, t15397: f64, t495: f64) -> (f64, f64, f64, f64, f64) {
    let t75143 = 0.2993560425465952141e-1_f64 * t739 * t75141;
    let t75145 = t14225 * t3352 * t8436;
    let t75148 = t1986 * t305 * t8441;
    let t75149 = t69619 * t75148;
    let t75152 = t14225 * t3352 * t8446;
    let t75154 = t15397 * t495;
    (t75143, t75145, t75149, t75152, t75154)
}
