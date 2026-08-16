//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1691/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1691(t5493: f64, t88: f64, t22473: f64, t5464: f64, t5488: f64, t6530: f64, t89: f64, t3788: f64, t6388: f64, t6936: f64, t1339: f64, t6420: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28007 = t88 * t5493;
    let t28012 = t22473 * t5464;
    let t28014 = t6530 * t5488;
    let t28030 = t89 * t5493;
    let t28057 = t3788 * t6388;
    let t28058 = t6936 * t28057;
    let t28060 = t1339 * t6420;
    (t28007, t28012, t28014, t28030, t28057, t28058, t28060)
}
