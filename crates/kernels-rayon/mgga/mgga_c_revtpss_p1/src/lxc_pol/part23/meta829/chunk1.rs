//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2688/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688(t16095: f64, t20100: f64, t43131: f64, t11922: f64, t20069: f64, t4899: f64, t20065: f64, t4892: f64, t15688: f64, t16584: f64, t15731: f64, t4879: f64) -> (f64, f64, f64, f64, f64) {
    let t67358 = t16095 * t43131 * t20100;
    let t67426 = t4899 * t11922 * t20069;
    let t67435 = t4892 * t11922 * t20065;
    let t67458 = t16584 * t15688;
    let t67473 = t4879 * t15731;
    (t67358, t67426, t67435, t67458, t67473)
}
