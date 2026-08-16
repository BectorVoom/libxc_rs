//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1124/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1124(t1569: f64, t2719: f64, t2713: f64, t3049: f64, t1108: f64, t3092: f64, t4265: f64, t242: f64, t3060: f64, t4246: f64, t1111: f64, t1562: f64, t9540: f64) -> (f64, f64, f64, f64, f64) {
    let t12429 = t1569 * t2719;
    let t12431 = t2713 * t3049 * t12429;
    let t12435 = t2713 * t1108 * t12429;
    let t12439 = t4265 * t3092 / 648.0_f64;
    let t12441 = t242 * t3060 * t4246;
    let t12443 = t1111 * t12441 / 2304.0_f64;
    let t12445 = t242 * t9540 * t1562;
    (t12431, t12435, t12439, t12443, t12445)
}
