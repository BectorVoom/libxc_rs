//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 938/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk938(t21815: f64, t5664: f64, t3738: f64, t6923: f64, t1464: f64, t12266: f64, t6928: f64, t3734: f64, t6932: f64, t12234: f64, t7042: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21816 = t21815 * t5664;
    let t21818 = t3738 * t6923;
    let t21819 = t1464 * t21818;
    let t21821 = t12266 * t6928;
    let t21822 = t1464 * t21821;
    let t21824 = t3734 * t6932;
    let t21825 = t1464 * t21824;
    let t21827 = t7042 * t12234;
    let t21828 = t21827 * t1385;
    (t21816, t21819, t21822, t21825, t21827, t21828)
}
