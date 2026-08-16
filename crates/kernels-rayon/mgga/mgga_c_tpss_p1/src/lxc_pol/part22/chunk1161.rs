//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1161/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1161(t125: f64, t4397: f64, t1233: f64, t3273: f64, t3327: f64, t4471: f64, t10151: f64, t4416: f64, t12863: f64, t4415: f64, t3240: f64, t4409: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12968 = t125 * t4397;
    let t12970 = t3273 * t12968 * t1233;
    let t12974 = t3273 * t4471 * t3327;
    let t12978 = t3273 * t4416 * t10151;
    let t12982 = t4415 * t12863 * t1233;
    let t12986 = t4415 * t4416 * t3327;
    let t12993 = 7.0_f64 / 72.0_f64 * t3240 * t4409;
    (t12970, t12974, t12978, t12982, t12986, t12993)
}
