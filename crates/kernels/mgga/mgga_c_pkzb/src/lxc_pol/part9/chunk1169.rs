//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1169/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1169<F: Float>(t1217: F, t18627: F, t3147: F, t6234: F, t6114: F, t1208: F, t18509: F, t18513: F, t6122: F, t898: F, t18679: F, t3160: F, t6226: F, t6502: F, t6320: F, t8219: F) -> (F, F, F, F, F, F, F, F) {
    let t22478 = 0.5848223622634646207e0 * t18627 * t1217;
    let t22480 = 0.10254018858216406658e4 * t3147 * t6234;
    let t22482 = 0.35089341735807877242e1 * t3147 * t6114;
    let t22487 = 0.91082604192152556044e5 * t898 * t18509 * t1208 * t18513 * t6122;
    let t22490 = 0.17315859105681463759e2 * t898 * t3160 * t18679;
    let t22492 = 0.5848223622634646207e0 * t3147 * t6226;
    let t22494 = 0.35089341735807877242e1 * t3147 * t6502;
    let t22496 = 6.0 * t8219 * t6320;
    (t22478, t22480, t22482, t22487, t22490, t22492, t22494, t22496)
}
