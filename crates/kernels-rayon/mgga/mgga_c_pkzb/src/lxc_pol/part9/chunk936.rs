//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 936/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk936(t6827: f64, t7195: f64, t7205: f64, t7215: f64, t45: f64, t1158: f64, t1824: f64, t3010: f64, t645: f64, t2873: f64, t5893: f64, t730: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7217 = t6827 + t7195 + t7205 + t7215;
    let t7218 = t45 * t7217;
    let t7219 = t1824 * t1158;
    let t7221 = t645 * t3010;
    let t7223 = t2873 * t5893;
    let t7225 = 0.17315859105681463759e2_f64 * t730 * t7223;
    (t7217, t7218, t7219, t7221, t7223, t7225)
}
