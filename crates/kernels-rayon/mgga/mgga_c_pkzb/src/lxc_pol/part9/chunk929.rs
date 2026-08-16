//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 929/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk929(t164: f64, t51: f64, t592: f64, t7084: f64, t2653: f64, t6939: f64, t179: f64, t1733: f64, t2645: f64, t5279: f64, t590: f64, t612: f64, t6972: f64, t6976: f64, t6981: f64, t6985: f64, t6988: f64, t6992: f64, t6995: f64, t6998: f64, t7001: f64, t7005: f64, t7009: f64) -> (f64, f64, f64) {
    let t7087 = t592 * t51 * t7084 * t164;
    let t7090 = t2653 * t6939;
    let t7091 = t179 * t7090;
    let t7094 = 0.17149607247227894789e-2_f64 * t1733 * t6972 + 0.85748036236139473944e-3_f64 * t1733 * t6976 - 0.42874018118069736972e-3_f64 * t2645 * t6981 - 0.85748036236139473944e-3_f64 * t612 * t6985 - 0.22675591804667994221e-1_f64 * t6988 - 0.25724410870841842183e-1_f64 * t612 * t6992 - 0.56688979511669985553e-2_f64 * t6995 - t6998 + 0.85748036236139473944e-2_f64 * t612 * t7001 + 0.42874018118069736972e-2_f64 * t612 * t7005 + t7009 - 0.21437009059034868486e-3_f64 * t590 * t7087 - 0.85748036236139473944e-2_f64 * t5279 * t7091;
    (t7087, t7091, t7094)
}
