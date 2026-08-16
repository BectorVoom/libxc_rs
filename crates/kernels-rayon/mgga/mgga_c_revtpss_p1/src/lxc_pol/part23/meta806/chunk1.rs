//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2639/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639(t231: f64, t2782: f64, t2783: f64, t62868: f64, t18729: f64, t2470: f64, t2798: f64, t2723: f64, t4503: f64, t62760: f64, t2482: f64, t6016: f64, t879: f64) -> (f64, f64, f64, f64) {
    let t62938 = t2782 * t2783 * t62868 * t231;
    let t62952 = t2798 * t18729 * t2470;
    let t62961 = t2782 * t4503 * t62760 * t2723;
    let t62967 = t2482 * t879 * t6016;
    (t62938, t62952, t62961, t62967)
}
