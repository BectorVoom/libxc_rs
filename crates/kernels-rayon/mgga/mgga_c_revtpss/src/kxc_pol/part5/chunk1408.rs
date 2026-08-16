//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1408/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1408(t1903: f64, t5774: f64, t4076: f64, t6918: f64, t72: f64, t686: f64, t3915: f64, t6889: f64, t786: f64, t1364: f64, t14100: f64, t5722: f64) -> (f64, f64, f64, f64) {
    let t22394 = t1903 * t5774;
    let t22395 = t4076 * t22394;
    let t22398 = t6918 * t72;
    let t22399 = t22398 * t686;
    let t22400 = t3915 * t22399;
    let t22404 = t786 * t6889;
    let t22405 = t22404 * t1364;
    let t22407 = t14100 * t5722;
    (t22395, t22400, t22405, t22407)
}
