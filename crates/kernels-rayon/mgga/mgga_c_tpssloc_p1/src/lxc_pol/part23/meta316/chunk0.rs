//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1074/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1074(t22032: f64, t457: f64, t460: f64, t974: f64, t1714: f64, t6144: f64, t1178: f64, t20217: f64, t1177: f64, t6138: f64, t4934: f64, t11516: f64, t20234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22034 = t457 * t22032 * t460;
    let t22035 = t974 * t22034;
    let t22038 = t6144 * t1714;
    let t22040 = t457 * t22038 * t460;
    let t22041 = t974 * t22040;
    let t22046 = t1178 * t20217;
    let t22047 = t1177 * t22046;
    let t22051 = t6138 * t1714 * t460;
    let t22052 = t4934 * t22051;
    let t22055 = t11516 * t20234;
    (t22034, t22035, t22038, t22040, t22041, t22046, t22047, t22051, t22052, t22055)
}
