//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1110/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1110(t179: f64, t2739: f64, t299: f64, t5672: f64, t17946: f64, t21454: f64, t54: f64, t7699: f64, t17867: f64, t2104: f64, t2932: f64, t2945: f64, t2947: f64, t5939: f64) -> (f64, f64, f64, f64, f64) {
    let t21714 = t299 * t179 * t5672 * t2739;
    let t21715 = 0.28582678745379824648e-3_f64 * t21714;
    let t21729 = t17946 * t21454;
    let t21787 = t54 * t7699;
    let t21862 = t2104 * t17867 * t2932;
    let t21863 = 0.28582678745379824648e-3_f64 * t21862;
    let t21870 = t2945 * t5939 * t2947;
    (t21715, t21729, t21787, t21863, t21870)
}
