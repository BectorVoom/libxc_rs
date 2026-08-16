//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1046/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1046(t1920: f64, t30874: f64, t1945: f64, t362: f64, t884: f64, t6784: f64, t8400: f64, t986: f64, t6800: f64, t6810: f64, t6799: f64, t1948: f64, t6768: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30876 = 0.54831135561607547883e-2_f64 * t1920 * t30874;
    let t30877 = t362 * t1945;
    let t30878 = t30877 * t884;
    let t30879 = t6784 * t30878;
    let t30882 = t986 * t8400;
    let t30885 = t6810 * t6800;
    let t30886 = t6799 * t30885;
    let t30889 = t1948 * t6768;
    (t30876, t30877, t30878, t30879, t30882, t30885, t30886, t30889)
}
