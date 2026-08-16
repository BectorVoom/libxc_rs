//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1960;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta429(t15281: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t1090: f64, t7319: f64, t4919: f64, t11531: f64, t11534: f64, t11537: f64, t11541: f64, t11591: f64, t15265: f64, t15269: f64, t15274: f64, t15278: f64, t3447: f64, t11583: f64, t3961: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15282, t15284, t15285, t15287, t15288, t15289, t15292) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1960(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let t15293 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1961(t11583, t3961);
    (t15282, t15284, t15285, t15287, t15288, t15289, t15292, t15293)
}
