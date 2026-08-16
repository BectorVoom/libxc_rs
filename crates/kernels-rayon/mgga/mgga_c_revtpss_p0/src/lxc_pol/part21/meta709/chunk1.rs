//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2539/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2539(t9775: f64, t9931: f64, t3989: f64, t9757: f64, t9761: f64, t9765: f64, t1353: f64, t13767: f64, t2661: f64, t3889: f64, t4010: f64, t240: f64, t9991: f64) -> (f64, f64, f64, f64, f64) {
    let t46598 = t9775 * t9931;
    let t46600 = t3989 * t9757;
    let t46602 = t9765 * t9761;
    let t46607 = t2661 * t13767 * t4010 * t3889 * t1353;
    let t46609 = t9991 * t240;
    (t46598, t46600, t46602, t46607, t46609)
}
