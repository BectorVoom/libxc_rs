//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1214/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1214(t5187: f64, t550: f64, t3805: f64, t5249: f64, t1307: f64, t6347: f64, t3870: f64, t820: f64, t19744: f64, t19871: f64, t5248: f64, t12369: f64) -> (f64, f64, f64, f64) {
    let t19989 = t550 * t5187;
    let t19991 = t3805 * t5249 * t19989;
    let t19994 = t6347 * t1307;
    let t19996 = t3870 * t820 * t19994;
    let t20000 = t5248 * t19871 * t19744;
    let t20004 = t3805 * t19871 * t12369;
    (t19991, t19996, t20000, t20004)
}
