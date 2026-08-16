//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 194/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk194(t568: f64, t616: f64, t615: f64, t578: f64, t580: f64, t583: f64, t590: f64, t603: f64, t611: f64, t612: f64) -> (f64, f64) {
    let t617 = t616 * t568;
    let t618 = t615 * t617;
    let t621 = -t578 - t580 * t583 / 48.0_f64 - 0.21437009059034868486e-3_f64 * t590 * t603 - t611 - 0.85748036236139473944e-3_f64 * t612 * t618;
    (t618, t621)
}
