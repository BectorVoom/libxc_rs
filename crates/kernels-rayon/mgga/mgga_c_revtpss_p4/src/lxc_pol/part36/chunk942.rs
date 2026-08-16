//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 942/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk942(t6918: f64, t72: f64, t686: f64, t3915: f64, t6889: f64, t786: f64, t1364: f64, t14100: f64, t5722: f64, t1357: f64, t6919: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t22398 = t6918 * t72;
    let t22399 = t22398 * t686;
    let t22400 = t3915 * t22399;
    let t22404 = t786 * t6889;
    let t22405 = t22404 * t1364;
    let t22407 = t14100 * t5722;
    let t22409 = t1357 * t6919;
    let t22410 = t689 * t22409;
    (t22399, t22400, t22405, t22407, t22410)
}
