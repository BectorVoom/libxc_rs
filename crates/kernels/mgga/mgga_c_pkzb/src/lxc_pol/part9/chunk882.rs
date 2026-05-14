//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 882/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk882<F: Float>(t164: F, t51: F, t592: F, t7084: F, t2653: F, t6939: F, t179: F, t1733: F, t2645: F, t5279: F, t590: F, t612: F, t6972: F, t6976: F, t6981: F, t6985: F, t6988: F, t6992: F, t6995: F, t6998: F, t7001: F, t7005: F, t7009: F) -> (F, F, F) {
    let t7087 = t592 * t51 * t7084 * t164;
    let t7090 = t2653 * t6939;
    let t7091 = t179 * t7090;
    let t7094 = 0.17149607247227894789e-2 * t1733 * t6972 + 0.85748036236139473944e-3 * t1733 * t6976 - 0.42874018118069736972e-3 * t2645 * t6981 - 0.85748036236139473944e-3 * t612 * t6985 - 0.22675591804667994221e-1 * t6988 - 0.25724410870841842183e-1 * t612 * t6992 - 0.56688979511669985553e-2 * t6995 - t6998 + 0.85748036236139473944e-2 * t612 * t7001 + 0.42874018118069736972e-2 * t612 * t7005 + t7009 - 0.21437009059034868486e-3 * t590 * t7087 - 0.85748036236139473944e-2 * t5279 * t7091;
    (t7087, t7091, t7094)
}
