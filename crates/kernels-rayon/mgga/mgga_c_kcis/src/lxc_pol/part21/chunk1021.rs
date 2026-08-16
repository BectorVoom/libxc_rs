//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1021/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1021(t15255: f64, t5324: f64, t11072: f64, t13480: f64, t5302: f64, t11081: f64, t5325: f64, t3514: f64, t330: f64, t5336: f64, t829: f64, t3515: f64) -> (f64, f64, f64, f64, f64) {
    let t15501 = t5324 * t15255;
    let t15502 = t11072 * t15501;
    let t15513 = t5302 * t13480;
    let t15516 = t11081 * t5325;
    let t15518 = t3514 * t15516 / 864.0_f64;
    let t15519 = t5336 * t330;
    let t15520 = t15519 * t829;
    let t15521 = t3515 * t15520;
    (t15501, t15502, t15513, t15518, t15521)
}
