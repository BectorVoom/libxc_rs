//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1150/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1150(t12836: f64, t3348: f64, t774: f64, t1625: f64, t3234: f64, t10077: f64, t1642: f64, t3245: f64, t9986: f64, t1244: f64, t12819: f64, t12825: f64, t12831: f64, t12835: f64, t3271: f64, t4413: f64, t9981: f64, t9991: f64, t9995: f64, t9997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12838 = t3348 * t774 * t12836;
    let t12841 = t1625 * t3234;
    let t12843 = t3348 * t774 * t12841;
    let t12846 = t10077 * t1642;
    let t12851 = t1625 * t3245;
    let t12853 = t9986 * t774 * t12851;
    let t12856 = 7.0_f64 / 4608.0_f64 * t9981 - 5.0_f64 / 384.0_f64 * t3271 * t12819 + t3271 * t12825 / 384.0_f64 - t4413 * t12831 / 192.0_f64 - t12835 + 5.0_f64 / 384.0_f64 * t1244 * t12838 + 5.0_f64 / 768.0_f64 * t1244 * t12843 - 119.0_f64 / 13824.0_f64 * t12846 - 35.0_f64 / 1152.0_f64 * t9991 - 119.0_f64 / 1728.0_f64 * t9995 + 7.0_f64 / 1152.0_f64 * t9997 - 5.0_f64 / 128.0_f64 * t1244 * t12853;
    (t12838, t12841, t12843, t12851, t12853, t12856)
}
