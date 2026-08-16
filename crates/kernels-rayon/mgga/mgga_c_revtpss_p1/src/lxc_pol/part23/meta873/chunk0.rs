//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2774/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774(t13845: f64, t13847: f64, t5675: f64, t73856: f64, t22107: f64, t9962: f64, t1399: f64, t22245: f64, t2661: f64, t3992: f64, t221: f64, t22287: f64) -> (f64, f64, f64, f64) {
    let t74469 = t13845 * t13847 * t73856 * t5675;
    let t74471 = t9962 * t22107;
    let t74475 = t2661 * t3992 * t22245 * t1399;
    let t74477 = t221 * t22287;
    (t74469, t74471, t74475, t74477)
}
