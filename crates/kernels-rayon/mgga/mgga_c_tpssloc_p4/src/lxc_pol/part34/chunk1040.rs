//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1040/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1040(t5488: f64, t6530: f64, t5493: f64, t89: f64, t3788: f64, t6388: f64, t6936: f64, t1339: f64, t6420: f64, t6417: f64, t6945: f64, t1827: f64, t26233: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28014 = t6530 * t5488;
    let t28030 = t89 * t5493;
    let t28057 = t3788 * t6388;
    let t28058 = t6936 * t28057;
    let t28060 = t1339 * t6420;
    let t28061 = t6936 * t28060;
    let t28063 = t6945 * t6417;
    let t28065 = t26233 * t1827;
    (t28014, t28030, t28057, t28058, t28060, t28061, t28063, t28065)
}
