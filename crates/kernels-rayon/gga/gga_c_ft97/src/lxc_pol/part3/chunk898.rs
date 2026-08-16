//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 898/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk898(t17855: f64, t3724: f64, t1127: f64, t709: f64, t679: f64, t689: f64, t3776: f64, t1614: f64, t694: f64, t3771: f64, t1109: f64, t4951: f64) -> (f64, f64, f64, f64, f64) {
    let t17856 = t3724 * t17855;
    let t17859 = t1127 * t709;
    let t17863 = t1127 * t679;
    let t17864 = t17863 * t689;
    let t17865 = t3776 * t17864;
    let t17868 = t694 * t1614;
    let t17870 = t3771 * t17868 * t679;
    let t17871 = t1109 * t709;
    let t17872 = t4951 * t17871;
    (t17856, t17859, t17865, t17870, t17872)
}
