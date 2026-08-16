//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 885/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk885(t44655: f64, t7474: f64, t1494: f64, t1970: f64, t209: f64, t236: f64, t3352: f64, t551: f64, t1971: f64, t615: f64, t7453: f64, t10072: f64, t7255: f64) -> (f64, f64, f64, f64) {
    let t44656 = t7474 * t44655;
    let t44662 = t1970 * t3352 * t236 * t551 * t1494 * t209;
    let t44668 = t7453 * t1971 * t236 * t615 * t1494 * t209;
    let t44670 = t7255 * t10072;
    (t44656, t44662, t44668, t44670)
}
