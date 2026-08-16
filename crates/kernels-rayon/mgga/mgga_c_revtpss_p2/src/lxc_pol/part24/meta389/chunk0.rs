//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1296/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1296(t159: f64, t2698: f64, t1544: f64, t1583: f64, t1868: f64, t1907: f64, t1501: f64, t1518: f64, t26: f64, t65: f64, t9163: f64, t99: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25273 = t2698 * t159;
    let t29598 = t1544 * t1583;
    let t30122 = t1868 * t1907;
    let t30138 = t1501 * t1518;
    let t33127 = 1.0_f64 / t65 / t26;
    let t36227 = t99 * t9163;
    (t25273, t29598, t30122, t30138, t33127, t36227)
}
