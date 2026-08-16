//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2685/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2685(t1025: f64, t371: f64, t6276: f64, t676: f64, t15749: f64, t4858: f64, t11789: f64, t20016: f64, t3205: f64, t6337: f64, t15666: f64, t1053: f64, t19463: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67186 = t1025 * t371 * t676 * t6276;
    let t67195 = t4858 * t15749;
    let t67199 = t11789 * t20016;
    let t67206 = t3205 * t371 * t676 * t6337;
    let t67213 = t4858 * t15666;
    let t67215 = t19463 * t1053;
    (t67186, t67195, t67199, t67206, t67213, t67215)
}
