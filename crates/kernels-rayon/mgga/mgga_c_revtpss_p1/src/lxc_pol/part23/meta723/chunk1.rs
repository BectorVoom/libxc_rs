//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2487/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2487(t46784: f64, t48908: f64, t124: f64, t5658: f64, t1889: f64, t46595: f64, t13850: f64, t2482: f64, t2668: f64, t4000: f64, t4010: f64, t808: f64) -> (f64, f64, f64, f64, f64) {
    let t48909 = t46784 * t48908;
    let t48919 = t124 * t5658;
    let t48947 = t46595 * t1889;
    let t48982 = t2482 * t4000 * t2668 * t13850;
    let t48999 = t808 * t4010;
    (t48909, t48919, t48947, t48982, t48999)
}
