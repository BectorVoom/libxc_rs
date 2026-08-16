//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1457/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1457(t1414: f64, t828: f64, t9628: f64, t221: f64, t3889: f64, t3979: f64, t3978: f64, t1408: f64, t2482: f64, t596: f64) -> (f64, f64, f64, f64) {
    let t9757 = t1414 * t828 * t9628;
    let t9761 = t3979 * t221 * t3889;
    let t9762 = t3978 * t9761;
    let t9765 = t2482 * t1408 * t596;
    (t9757, t9761, t9762, t9765)
}
