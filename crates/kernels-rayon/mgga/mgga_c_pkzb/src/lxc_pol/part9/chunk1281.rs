//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1281/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1281(t1217: f64, t18627: f64, t3147: f64, t6234: f64, t6114: f64, t1208: f64, t18509: f64, t18513: f64, t6122: f64, t898: f64, t18679: f64, t3160: f64) -> (f64, f64, f64, f64, f64) {
    let t22478 = 0.5848223622634646207e0_f64 * t18627 * t1217;
    let t22480 = 0.10254018858216406658e4_f64 * t3147 * t6234;
    let t22482 = 0.35089341735807877242e1_f64 * t3147 * t6114;
    let t22487 = 0.91082604192152556044e5_f64 * t898 * t18509 * t1208 * t18513 * t6122;
    let t22490 = 0.17315859105681463759e2_f64 * t898 * t3160 * t18679;
    (t22478, t22480, t22482, t22487, t22490)
}
