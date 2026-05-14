//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 797/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk797<F: Float>(t179: F, t568: F, t8914: F, t1733: F, t2645: F, t5244: F, t590: F, t612: F, t8823: F, t8827: F, t8832: F, t8835: F, t8837: F, t8891: F, t8894: F, t8897: F, t8901: F, t8906: F, t8911: F) -> (F, F) {
    let t8916 = t179 * t8914 * t568;
    let t8919 = -0.25724410870841842183e-1 * t612 * t8823 + 0.85748036236139473944e-2 * t612 * t8827 + 0.42874018118069736972e-2 * t612 * t8832 + 0.10003937560882938627e-2 * t8835 - 0.20007875121765877254e-2 * t8837 - 0.21437009059034868486e-3 * t590 * t8891 + 0.10003937560882938627e-2 * t8894 + 0.17149607247227894789e-2 * t1733 * t8897 + 0.17149607247227894789e-2 * t1733 * t8901 + 0.85748036236139473944e-3 * t1733 * t8906 - 0.21437009059034868486e-3 * t2645 * t8911 - 0.17149607247227894789e-2 * t5244 * t8916;
    (t8916, t8919)
}
