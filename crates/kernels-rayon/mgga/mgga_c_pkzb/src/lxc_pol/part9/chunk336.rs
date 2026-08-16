//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 336/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk336(t1143: f64, t790: f64, t1134: f64, t307: f64, t311: f64, t1072: f64, t1086: f64, t1112: f64, t1114: f64, t1118: f64, t135: f64, t273: f64, t805: f64) -> (f64, f64, f64) {
    let t1144 = t790 * t1143;
    let t1147 = 0.65854491829355115987e0_f64 * t1134 * t311 - 0.65854491829355115987e0_f64 * t307 * t1144;
    let t1151 = t1147 * t135 * t273 * t805 - t1072 + t1086 + t1112 + t1114 - t1118;
    (t1144, t1147, t1151)
}
