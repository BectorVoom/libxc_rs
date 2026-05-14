//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1258/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1258<F: Float>(t24523: F, t24714: F, t1769: F, t8832: F, t164: F, t1692: F, t1733: F, t179: F, t20436: F, t20592: F, t24487: F, t24489: F, t2575: F, t2600: F, t2646: F, t2660: F, t2661: F, t51: F, t5279: F, t568: F, t590: F, t592: F, t612: F, t6853: F, t6990: F, t8817: F, t8821: F, t8830: F) -> (F, F) {
    let t24715 = t24523 + t24714;
    let t24729 = t1769 * t8832;
    let t24749 = 0.11337795902333997111e-1 * t24487 - 0.56688979511669985553e-2 * t24489 + 0.32012600194825403606e-1 * t20436 - 0.21437009059034868486e-3 * t590 * t592 * t51 * t24715 * t164 - 0.25724410870841842183e-1 * t612 * t6990 * t8821 * t1692 + 0.85748036236139473944e-2 * t612 * t2660 * t2661 * t6853 - 0.40015750243531754508e-1 * t24729 + 0.85748036236139473944e-2 * t612 * t2660 * t51 * t8817 * t568 + 0.42874018118069736972e-2 * t612 * t2660 * t8830 * t1692 - 0.17149607247227894789e-1 * t5279 * t179 * t2600 * t20592 + 0.34299214494455789578e-2 * t1733 * t179 * t2646 * t164 * t2575;
    (t24715, t24749)
}
