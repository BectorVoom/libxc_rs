//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1191/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1191(t17885: f64, t309: f64, t441: f64, t1128: f64, t17709: f64, t8921: f64, t17713: f64, t2586: f64, t1133: f64, t1121: f64, t17857: f64, t17719: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54599 = t17885 * t309;
    let t54600 = t441 * t54599;
    let t54613 = t8921 * t1128 * t17709;
    let t54615 = t2586 * t17713;
    let t54616 = t1133 * t54615;
    let t54619 = t1121 * t1128 * t17857;
    let t54621 = t2586 * t17719;
    (t54599, t54600, t54613, t54615, t54616, t54619, t54621)
}
