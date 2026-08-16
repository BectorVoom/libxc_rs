//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 974/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk974(t508: f64, t8406: f64, t569: f64, t1911: f64, t2198: f64, t1312: f64, t2199: f64, t2201: f64, t4248: f64, t651: f64, t7732: f64, t7889: f64, t8393: f64) -> (f64, f64, f64, f64) {
    let t8407 = t508 * t8406;
    let t8411 = t8406 * t569;
    let t8413 = t2198 * t1911;
    let t8416 = 2.0_f64 * t1312 * t8411 + 2.0_f64 * t1312 * t8413 - 2.0_f64 * t2199 * t4248 - 2.0_f64 * t2199 * t7732 + 2.0_f64 * t2201 * t4248 + 2.0_f64 * t2201 * t7889 - 2.0_f64 * t651 * t8393 - 2.0_f64 * t651 * t8407;
    (t8407, t8411, t8413, t8416)
}
