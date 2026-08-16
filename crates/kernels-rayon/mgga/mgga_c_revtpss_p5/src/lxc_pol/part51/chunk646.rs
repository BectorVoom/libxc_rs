//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 646/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk646(t651: f64, t7003: f64, t2007: f64, t670: f64, t30: f64, t775: f64, t1949: f64, t212: f64, t780: f64, t689: f64, t1950: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7005 = 2.0_f64 * t651 * t7003;
    let t7007 = t2007 * t670;
    let t7010 = t30 * t775;
    let t7014 = t212 * t1949;
    let t7015 = t7014 * t780;
    let t7017 = 0.54878743191129263322e-2_f64 * t689 * t7015;
    let t7018 = t786 * t1950;
    (t7005, t7007, t7010, t7014, t7015, t7017, t7018)
}
