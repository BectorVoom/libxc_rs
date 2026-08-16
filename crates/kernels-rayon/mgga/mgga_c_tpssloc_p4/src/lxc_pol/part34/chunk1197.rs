//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1197/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1197(t20442: f64, t22833: f64, t2002: f64, t20595: f64, t559: f64, t1985: f64, t26193: f64, t28205: f64, t28209: f64, t6888: f64, t20608: f64, t6889: f64, t80640: f64) -> (f64, f64, f64, f64, f64) {
    let t107198 = t22833 * t20442;
    let t107205 = t20595 * t2002 * t559;
    let t107214 = t1985 * t26193 * t28205;
    let t107230 = t6888 * t26193 * t28209;
    let t107238 = t1985 * t6889 * t80640 * t20608;
    (t107198, t107205, t107214, t107230, t107238)
}
