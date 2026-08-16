//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 929/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk929(t1212: f64, t1971: f64, t209: f64, t236: f64, t615: f64, t7453: f64, t1240: f64, t1475: f64, t1182: f64, t570: f64, t1184: f64, t515: f64, t7365: f64) -> (f64, f64, f64, f64) {
    let t40420 = t7453 * t1971 * t236 * t615 * t1212 * t209;
    let t40425 = t7453 * t1971 * t236 * t1475 * t1240;
    let t40427 = t570 * t1182;
    let t40431 = t7365 * t1971 * t515 * t40427 * t1184;
    (t40420, t40425, t40427, t40431)
}
