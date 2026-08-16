//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1049/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1049(t1936: f64, t33602: f64, t6985: f64, t7741: f64, t1501: f64, t8453: f64, t8692: f64, t4248: f64, t8460: f64, t7889: f64, t7742: f64, t8634: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33635 = t33602 * t1936;
    let t33637 = t6985 * t7741;
    let t33639 = t1501 * t8453;
    let t33640 = 2.0_f64 * t33639;
    let t33642 = 4.0_f64 * t8692 * t7741;
    let t33643 = t4248 * t8460;
    let t33644 = 2.0_f64 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0_f64 * t33645;
    let t33650 = 4.0_f64 * t8634 * t7742;
    (t33635, t33637, t33639, t33640, t33642, t33643, t33644, t33645, t33646, t33650)
}
