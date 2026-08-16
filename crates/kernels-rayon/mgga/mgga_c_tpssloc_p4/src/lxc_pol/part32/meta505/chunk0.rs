//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1830/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1830(t26193: f64, t6891: f64, t6888: f64, t22674: f64, t7691: f64, t22892: f64, t6883: f64, t7701: f64, t5353: f64, t6906: f64, t6889: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26194 = t26193 * t6891;
    let t26195 = t6888 * t26194;
    let t26197 = t22674 * t7691;
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26202 = t6906 * t5353;
    let t26203 = t6889 * t26202;
    let t26204 = t1985 * t26203;
    (t26194, t26195, t26197, t26198, t26200, t26202, t26203, t26204)
}
