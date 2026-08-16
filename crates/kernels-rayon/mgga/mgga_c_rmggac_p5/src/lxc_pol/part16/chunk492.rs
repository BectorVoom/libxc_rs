//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 492/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk492(t1480: f64, t1486: f64, t1488: f64, t1491: f64, t1878: f64, t1882: f64, t1885: f64, t206: f64, t207: f64, t470: f64, t473: f64, t600: f64, t602: f64, t6218: f64, t6224: f64, t6232: f64, t6235: f64, t6241: f64, t6244: f64) -> f64 {
    let t6247 = 6.0_f64 * t1480 * t602 + 60.0_f64 * t1486 * t6232 - 24.0_f64 * t1486 * t6235 - 12.0_f64 * t1486 * t6241 - 24.0_f64 * t1488 * t6224 + 6.0_f64 * t1491 * t600 + 3.0_f64 * t1878 * t473 - 12.0_f64 * t1882 * t470 + 3.0_f64 * t1885 * t470 + 3.0_f64 * t206 * t6244 - t207 * t6218;
    t6247
}
