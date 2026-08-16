//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1164;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1165;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1166;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1167;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1168;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1169;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1170;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1171;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta192<F: Float>(t1021: F, t248: F, t4650: F, t1020: F, t1025: F, t1041: F, t1046: F, t1618: F, t1622: F, t3104: F, t3109: F, t3114: F, t3117: F, t3140: F, t3156: F, t3160: F, t3163: F, t378: F, t4617: F, t4622: F, t4625: F, t4631: F, t4636: F, t4641: F, t4644: F, t4613: F, t349: F, t1626: F, t225: F, t1065: F, t1634: F, t3174: F, t1057: F, t4639: F, t1022: F, t3188: F, t1629: F, t1049: F, t1615: F, t1060: F, t381: F, t4649: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t4652 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1164::<F>(t1021, t248, t4650);
        let t4656 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1165::<F>(t1020, t1025, t1041, t1046, t1618, t1622, t3104, t3109, t3114, t3117, t3140, t3156, t3160, t3163, t378, t4617, t4622, t4625, t4631, t4636, t4641, t4644, t4652);
        let t4657 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1166::<F>(t4613, t4656);
        let (t4658, t4660) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1167::<F>(t349, t4657, t1626, t225);
        let t4665 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1168::<F>(t1065, t1634, t3174);
        let t4669 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1169::<F>(t1057, t4639);
        let t4673 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1170::<F>(t1022, t3188);
        let (t4674, t4677) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1171::<F>(t1629, t4673, t1049, t1615);
        let (t4678, t4680) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1172::<F>(t1060, t4677, t381, t4649);
    (t4652, t4657, t4658, t4660, t4665, t4669, t4673, t4674, t4677, t4678, t4680)
}
