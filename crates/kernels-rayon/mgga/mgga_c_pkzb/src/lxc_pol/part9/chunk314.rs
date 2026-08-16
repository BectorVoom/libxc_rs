//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 314/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk314(t1034: f64, t51: f64, t164: f64, t592: f64, t1020: f64, t616: f64, t615: f64, t1025: f64, t578: f64, t580: f64, t590: f64, t611: f64, t612: f64) -> (f64, f64, f64) {
    let t1035 = t51 * t1034;
    let t1037 = t592 * t1035 * t164;
    let t1040 = t616 * t1020;
    let t1041 = t615 * t1040;
    let t1044 = -t578 - t580 * t1025 / 48.0_f64 - 0.21437009059034868486e-3_f64 * t590 * t1037 - t611 - 0.85748036236139473944e-3_f64 * t612 * t1041;
    (t1037, t1041, t1044)
}
