//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 526/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk526(t4295: f64, t467: f64, t488: f64, t3906: f64, t492: f64, t500: f64, t470: f64, t3777: f64, t498: f64, t493: f64, t1492: f64, t1496: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4296 = t4295 * t467;
    let t4297 = t4296 * sigma0;
    let t4298 = t4297 * t488;
    let t4300 = t3906 * t467;
    let t4301 = t4300 * t492;
    let t4302 = t4301 * t500;
    let t4304 = 1.0_f64 / t470;
    let t4305 = t4304 * t3777;
    let t4306 = t498 * t4305;
    let t4307 = t493 * t4306;
    let t4309 = t1492 * t1496;
    (t4297, t4298, t4300, t4301, t4302, t4304, t4305, t4306, t4307, t4309)
}
