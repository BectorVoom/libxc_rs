//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 483/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk483(t1670: f64, t1118: f64, t3264: f64, t1661: f64, t3270: f64, t3274: f64, t4721: f64, t5973: f64, t5977: f64, t5981: f64, t1100: f64, t3287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5988 = t1670 * t1670;
    let t5989 = t5988 * t1118;
    let t5991 = 2.0_f64 * t3264 * t5989;
    let t5992 = t1661 * t1661;
    let t5993 = t3270 * t5992;
    let t5999 = t3274 - 2.0_f64 / 9.0_f64 * t4721 - 2.0_f64 / 9.0_f64 * t5973 + 2.0_f64 / 3.0_f64 * t5977 + t5981 / 3.0_f64;
    let t6000 = t1100 * t5999;
    let t6006 = t3287 * t5992;
    (t5988, t5991, t5993, t5999, t6000, t6006)
}
