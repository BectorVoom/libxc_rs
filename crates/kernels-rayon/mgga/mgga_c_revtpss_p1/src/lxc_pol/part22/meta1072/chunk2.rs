//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3844/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3844(t1388: f64, t1390: f64, t1410: f64, t3889: f64, t4002: f64, t4012: f64, t46598: f64, t46602: f64, t46620: f64, t46633: f64, t46645: f64, t6816: f64, t73923: f64, t73927: f64, t73929: f64, t73937: f64, t73942: f64, t828: f64) -> f64 {
    let t73947 = -0.76220476654346199061e-4_f64 * t46598 + 0.54208002996571016772e-3_f64 * t46602 + 0.14450132032386466905e-2_f64 * t46620 + 0.1133779590233399711e0_f64 * t46633 - 0.10276933901433255263e-1_f64 * t46645 - 0.57165357490759649296e-4_f64 * t73923 + 0.14291339372689912324e-4_f64 * t73927 + 0.11337795902333997111e-1_f64 * t73929 + 0.42874018118069736972e-2_f64 * t1410 * t4012 * t828 * t6816 * t3889 - 0.42874018118069736972e-3_f64 * t1388 * t1390 * t828 * t73937 + 0.85748036236139473944e-3_f64 * t4002 * t1390 * t828 * t73942;
    t73947
}
