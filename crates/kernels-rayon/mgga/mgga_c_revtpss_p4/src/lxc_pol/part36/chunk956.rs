//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 956/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk956(t117: f64, t22746: f64, t1312: f64, t1518: f64, t18245: f64, t22633: f64, t22639: f64, t4248: f64, t5920: f64, t7889: f64, t13584: f64, t22186: f64) -> (f64, f64, f64, f64) {
    let t22747 = t22746 * t117;
    let t22758 = 2.0_f64 * t1312 * t22633 + 6.0_f64 * t1518 * t18245 + 6.0_f64 * t4248 * t5920 + 6.0_f64 * t5920 * t7889 + 6.0_f64 * t22639 + t22747;
    let t22762 = 60.0_f64 * t13584;
    let t22763 = 0.54934341918019635162e-3_f64 * t22186;
    (t22747, t22758, t22762, t22763)
}
