//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 942/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk942(t17412: f64, t4262: f64, t1539: f64, t5999: f64, t16665: f64, t6011: f64, t6010: f64, t2042: f64, t4256: f64, t4255: f64, t2035: f64, t4270: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17413 = t17412 * t4262;
    let t17415 = t5999 * t1539;
    let t17417 = t6011 * t16665;
    let t17418 = t6010 * t17417;
    let t17420 = t2042 * t4256;
    let t17421 = t4255 * t17420;
    let t17423 = t2035 * t4270;
    (t17413, t17415, t17417, t17418, t17421, t17423)
}
