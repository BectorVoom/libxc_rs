//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1912;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1913;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1914;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta456(t1706: f64, t3428: f64, t1184: f64, t460: f64, t4928: f64, t4934: f64, t1714: f64, t3469: f64, t1178: f64, t12606: f64, t1177: f64, t135: f64, t457: f64, t4936: f64, t1174: f64, t3431: f64, t4912: f64, t1090: f64, t7319: f64, t4919: f64, t11531: f64, t11534: f64, t11537: f64, t11541: f64, t11591: f64, t3447: f64, t11583: f64, t3961: f64, t3449: f64, t11529: f64, t1709: f64, t3475: f64, t3432: f64, t4889: f64, t3450: f64, t3966: f64, t14749: f64, t4908: f64, t3448: f64, t3451: f64, t11579: f64, t11584: f64, t3443: f64, t3457: f64, t3461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15265, t15268, t15269, t15273, t15274, t15277, t15278, t15281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1912(t1706, t3428, t1184, t460, t4928, t4934, t1714, t3469, t1178, t12606, t1177, t135, t457);
        let (t15288, t15292) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1913(t15281, t4936, t1174, t3431, t4912, t1090, t7319, t4919, t11531, t11534, t11537, t11541, t11591, t15265, t15269, t15274, t15278, t3447);
        let (t15293, t15294, t15300, t15303, t15304, t15307) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1914(t11583, t3961, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889);
        let (t15313, t15320, t15330) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1915(t3450, t3966, t3449, t14749, t4908, t3448, t4928, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t3443, t3447, t3457, t3461, t4889);
    (t15268, t15273, t15277, t15281, t15288, t15292, t15293, t15303, t15313, t15320, t15330)
}
