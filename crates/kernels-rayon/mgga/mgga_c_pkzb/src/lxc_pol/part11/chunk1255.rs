//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1255/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1255(t3605: f64, t730: f64, t7527: f64, t25671: f64, t2852: f64, t3618: f64, t7560: f64, t7299: f64, t9351: f64, t20982: f64, t9531: f64, t2865: f64, t9465: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30764 = 0.35089341735807877242e1_f64 * t730 * t7527 * t3605;
    let t30767 = 0.51947577317044391277e2_f64 * t730 * t25671 * t2852;
    let t30769 = 0.35089341735807877242e1_f64 * t7560 * t3618;
    let t30772 = 0.51947577317044391277e2_f64 * t730 * t9351 * t7299;
    let t30775 = 0.30762056574649219974e4_f64 * t730 * t9531 * t20982;
    let t30778 = 0.35089341735807877242e1_f64 * t730 * t2865 * t9465;
    (t30764, t30767, t30769, t30772, t30775, t30778)
}
