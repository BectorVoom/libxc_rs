//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 792/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk792<F: Float>(t37939: F, t66: F, t22547: F, t1620: F, t6: F, t7984: F, t7988: F, t5517: F, t5544: F, t11153: F, t11232: F, t11241: F, t1594: F, t1599: F, t1617: F, t1618: F, t1624: F, t1631: F, t1656: F, t1665: F, t1683: F, t1685: F, t1701: F, t1702: F, t1751: F, t22548: F, t3076: F, t372: F, t374: F, t37509: F, t37578: F, t37582: F, t37627: F, t37685: F, t37854: F, t37899: F, t37905: F, t37908: F, t37931: F, t37935: F, t7877: F, t7879: F, t79: F, t7906: F, t8015: F, t8042: F, t8154: F, t8161: F, t8169: F) -> (F,) {
    let t37940 = t37939 * t66;
    let t37941 = t22547 * t37940;
    let t37943 = t7984 * t6 * t1620;
    let t37947 = t7988 * t6 * t1620;
    let t37952 = t5517 * t5544;
    let t37957 = 0.279058811357253504e-1 * t8042 * t1631 * t37854 - 0.139529405678626752e-1 * t1624 * t1631 * t37582 - 0.22941158433316392859e1 * t79 * t37685 + 0.60826526699468500834e-9 * t79 * t37899 + 0.43019436846165064134e-1 * t79 * t37905 + 0.558117622714507008e-1 * t11241 * t37908 * t8169 - 0.279058811357253504e-1 * t11232 * t37908 * t8161 - 0.558117622714507008e-1 * t7877 * t11153 * t7879 + 0.69764702839313376e-1 * t1624 * t374 * t1656 * t1751 + 0.20279640676073749279e-3 * t1594 * t37627 * t1599 - 0.69716604262587839785e-3 * t372 * t7906 * t37578 - 0.45048092923603098705e0 * t1665 * t1683 - 0.38995437477448399246e-5 * t3076 * t8154 * t37931 - 0.12803864807119409228e-1 * t1617 * t1618 * t37935 + 0.38465647900339007384e-4 * t37941 * t37943 - 0.19232823950169503692e-4 * t22548 * t37947 - 0.52379446938215765024e-3 * t8015 * t37509 + 0.1422571355482203117e0 * t37952 * t1701 * t1702 * t1685;
    (t37957,)
}
