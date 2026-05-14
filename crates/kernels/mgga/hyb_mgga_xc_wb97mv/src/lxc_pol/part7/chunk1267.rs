//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1267/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1267<F: Float>(t11068: F, t11123: F, t11138: F, t11155: F, t11158: F, t11159: F, t11182: F, t11185: F, t1365: F, t22428: F, t2262: F, t2263: F, t22749: F, t2278: F, t2284: F, t2301: F, t2302: F, t2317: F, t2323: F, t26839: F, t4216: F, t4230: F, t4243: F, t4246: F, t6871: F, t6876: F, t6923: F, t6929: F, t6966: F, t6977: F, t6982: F, t827: F, t9000: F, t9061: F, t9077: F, t9130: F, t9135: F, t9155: F, t9159: F) -> (F,) {
    let t31052 = 12.0 * t9061 * t9130 + 0.70178683471615754484e1 * t9077 * t9135 + 0.35089341735807877242e1 * t2323 * t4243 * t2302 + 0.6233709278045326953e3 * t6966 * t4246 * t2302 + 6.0 * t2284 * t4216 * t2263 - 4.0 * t6923 * t11155 - 4.0 * t2262 * t11068 * t827 - 2.0 * t2262 * t4216 * t2278 - 0.19298375398431042081e3 * t6876 * t11158 * t2263 + 0.64327917994770140268e2 * t6871 * t11159 - 0.20779030926817756511e3 * t22749 * t11123 - 0.10389515463408878255e3 * t6982 * t4246 * t2317 - 0.12304822629859687989e5 * t22428 * t11138 * t2302 + 0.34631718211362927517e2 * t9077 * t9155 + 0.20508037716432813315e4 * t26839 * t9159 + 0.70178683471615754484e1 * t6977 * t11182 + 0.35089341735807877242e1 * t2323 * t4230 * t2317 - 0.46785788981077169656e1 * t6929 * t11185 - 0.23392894490538584828e1 * t2301 * t1365 * t9000;
    (t31052,)
}
