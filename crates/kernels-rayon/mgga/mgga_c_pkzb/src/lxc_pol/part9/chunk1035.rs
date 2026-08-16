//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1035/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1035(t133: f64, t8309: f64, t945: f64, t2393: f64, t8515: f64, t2970: f64, t8445: f64, t3258: f64, t6455: f64, t7832: f64, t8451: f64, t1245: f64, t2363: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8532 = t8309 * t133;
    let t8533 = t8532 * t945;
    let t8536 = t2393 * t8515;
    let t8539 = t2970 * t8445;
    let t8542 = t6455 * t3258;
    let t8543 = t7832 * t8451;
    let t8546 = t2363 * t1245;
    (t8532, t8533, t8536, t8539, t8542, t8543, t8546)
}
