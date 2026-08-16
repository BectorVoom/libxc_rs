//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 976/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk976(t14379: f64, t470: f64, t486: f64, t494: f64, t391: f64, t79: f64, t499: f64, t493: f64, t13949: f64, t4204: f64, t4203: f64, t1505: f64, t4181: f64) -> (f64, f64, f64, f64) {
    let t14380 = t14379 * t470;
    let t14381 = t486 * t14380;
    let t14383 = t494 * t494;
    let t14386 = 1.0_f64 / t391 / t14383 * t79;
    let t14387 = t14386 * t499;
    let t14388 = t493 * t14387;
    let t14390 = t4204 * t13949;
    let t14391 = t4203 * t14390;
    let t14393 = t4181 * t1505;
    (t14381, t14388, t14391, t14393)
}
