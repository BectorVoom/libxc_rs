//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 592/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk592(t3928: f64, t8410: f64, t5888: f64, t7577: f64, t739: f64, t1469: f64, t236: f64, t1971: f64, t7365: f64, t1475: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8411 = t3928 * t8410;
    let t8413 = t7577 * t5888;
    let t8414 = t739 * t8413;
    let t8416 = t236 * t1469;
    let t8417 = t1971 * t8416;
    let t8418 = t7365 * t8417;
    let t8420 = t1475 * t498;
    (t8411, t8413, t8414, t8417, t8418, t8420)
}
