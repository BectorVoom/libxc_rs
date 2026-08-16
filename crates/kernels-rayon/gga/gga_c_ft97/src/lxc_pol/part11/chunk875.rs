//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 875/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk875(t11153: f64, t11232: f64, t11241: f64, t1594: f64, t1599: f64, t1617: f64, t1618: f64, t1624: f64, t1631: f64, t1656: f64, t1665: f64, t1683: f64, t1685: f64, t1701: f64, t1702: f64, t1751: f64, t22548: f64, t3076: f64, t372: f64, t374: f64, t37509: f64, t37578: f64, t37582: f64, t37627: f64, t37685: f64, t37854: f64, t37899: f64, t37905: f64, t37908: f64, t37931: f64, t37935: f64, t37941: f64, t37943: f64, t37947: f64, t37952: f64, t7877: f64, t7879: f64, t79: f64, t7906: f64, t8015: f64, t8042: f64, t8154: f64, t8161: f64, t8169: f64) -> f64 {
    let t37957 = 0.279058811357253504e-1_f64 * t8042 * t1631 * t37854 - 0.139529405678626752e-1_f64 * t1624 * t1631 * t37582 - 0.22941158433316392859e1_f64 * t79 * t37685 + 0.60826526699468500834e-9_f64 * t79 * t37899 + 0.43019436846165064134e-1_f64 * t79 * t37905 + 0.558117622714507008e-1_f64 * t11241 * t37908 * t8169 - 0.279058811357253504e-1_f64 * t11232 * t37908 * t8161 - 0.558117622714507008e-1_f64 * t7877 * t11153 * t7879 + 0.69764702839313376e-1_f64 * t1624 * t374 * t1656 * t1751 + 0.20279640676073749279e-3_f64 * t1594 * t37627 * t1599 - 0.69716604262587839785e-3_f64 * t372 * t7906 * t37578 - 0.45048092923603098705e0_f64 * t1665 * t1683 - 0.38995437477448399246e-5_f64 * t3076 * t8154 * t37931 - 0.12803864807119409228e-1_f64 * t1617 * t1618 * t37935 + 0.38465647900339007384e-4_f64 * t37941 * t37943 - 0.19232823950169503692e-4_f64 * t22548 * t37947 - 0.52379446938215765024e-3_f64 * t8015 * t37509 + 0.1422571355482203117e0_f64 * t37952 * t1701 * t1702 * t1685;
    t37957
}
