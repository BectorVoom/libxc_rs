//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 559/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk559(t2164: f64, t702: f64, t638: f64, t639: f64, t2231: f64, t640: f64, t13970: f64, t13976: f64, t2012: f64, t2265: f64, t2010: f64, t3194: f64, t4965: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14559 = t2164 * t702;
    let t14561 = t638 * t639 * t14559;
    let t14562 = 0.15243824895787514157e-3_f64 * t14561;
    let t14563 = t640 * t2231;
    let t14565 = t638 * t639 * t14563;
    let t14566 = 0.15243824895787514157e-3_f64 * t14565;
    let t14570 = 0.68186654135613354325e-2_f64 * t13970;
    let t14571 = 0.85129199786595678799e-5_f64 * t13976;
    let t14572 = t2012 * t2265;
    let t14573 = t2010 * t14572;
    let t14574 = 0.36021158228745895953e-3_f64 * t14573;
    let t14576 = t4965 * t3194;
    (t14559, t14562, t14563, t14566, t14570, t14571, t14572, t14574, t14576)
}
