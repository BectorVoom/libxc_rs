//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1411/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1411(t46784: f64, t48908: f64, t1889: f64, t46595: f64, t1873: f64, t46651: f64, t13800: f64, t46670: f64, t3964: f64, t5617: f64, t9732: f64, t46888: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48909 = t46784 * t48908;
    let t48947 = t46595 * t1889;
    let t49030 = t46651 * t1873;
    let t49087 = t46670 * t13800;
    let t49090 = t3964 * t9732 * t5617;
    let t49105 = t46888 * t48908;
    (t48909, t48947, t49030, t49087, t49090, t49105)
}
