//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 876/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk876(t13143: f64, t3544: f64, t306: f64, t3529: f64, t459: f64, t1175: f64, t3575: f64, t3530: f64, t425: f64, t1364: f64, t12983: f64, t5895: f64) -> (f64, f64, f64, f64) {
    let t13144 = t3544 * t13143;
    let t13148 = t3529 * t306 * t459;
    let t13149 = t3575 * t1175;
    let t13150 = t13148 * t13149;
    let t13153 = t3530 * t425;
    let t13154 = t3575 * t1364;
    let t13155 = t13153 * t13154;
    let t13158 = t5895 * t12983;
    (t13144, t13150, t13155, t13158)
}
