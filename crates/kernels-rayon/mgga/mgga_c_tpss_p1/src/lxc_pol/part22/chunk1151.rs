//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1151/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1151(t1629: f64, t3245: f64, t762: f64, t10160: f64, t1630: f64, t125: f64, t4459: f64, t3273: f64, t3275: f64, t4415: f64, t4417: f64, t3332: f64, t4471: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12858 = t762 * t1629 * t3245;
    let t12861 = t10160 * t1630;
    let t12863 = t125 * t4459;
    let t12865 = t3273 * t12863 * t3275;
    let t12869 = t4415 * t12863 * t4417;
    let t12873 = t3273 * t4471 * t3332;
    (t12858, t12861, t12863, t12865, t12869, t12873)
}
