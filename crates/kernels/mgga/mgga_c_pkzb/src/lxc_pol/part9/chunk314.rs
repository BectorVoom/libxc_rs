//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 314/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk314<F: Float>(t1034: F, t51: F, t164: F, t592: F, t1020: F, t616: F, t615: F, t1025: F, t578: F, t580: F, t590: F, t611: F, t612: F) -> (F, F, F) {
    let t1035 = t51 * t1034;
    let t1037 = t592 * t1035 * t164;
    let t1040 = t616 * t1020;
    let t1041 = t615 * t1040;
    let t1044 = -t578 - t580 * t1025 / F::cast_from(48.0_f64) - F::cast_from(0.21437009059034868486e-3_f64) * t590 * t1037 - t611 - F::cast_from(0.85748036236139473944e-3_f64) * t612 * t1041;
    (t1037, t1041, t1044)
}
