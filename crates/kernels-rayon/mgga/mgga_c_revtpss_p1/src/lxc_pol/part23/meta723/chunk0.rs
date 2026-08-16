//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2486/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2486(t1389: f64, t14230: f64, t2735: f64, t46801: f64, t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13760: f64, t9765: f64, t268: f64, t5617: f64) -> (f64, f64, f64, f64, f64) {
    let t48876 = t2735 * t46801 * t1389 * t14230;
    let t48877 = 0.15246000842785598467e-4_f64 * t48876;
    let t48879 = t9793 * t40763 * t5609;
    let t48881 = t9775 * t13830;
    let t48904 = t9765 * t13760;
    let t48905 = 0.16262400898971305032e-2_f64 * t48904;
    let t48908 = t5617 * t268;
    (t48877, t48879, t48881, t48905, t48908)
}
