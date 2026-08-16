//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1517/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1517(t15691: f64, t16229: f64, t1047: f64, t1063: f64, t11656: f64, t11977: f64, t15700: f64, t16190: f64, t16196: f64, t16201: f64, t16205: f64, t16210: f64, t16218: f64, t16220: f64, t16223: f64, t16226: f64, t1671: f64, t3169: f64, t4825: f64, t4869: f64) -> f64 {
    let t16230 = t15691 * t16229;
    let t16233 = -0.22866142996303859718e-2_f64 * t16190 * t1047 + 0.15244095330869239812e-2_f64 * t11656 * t4825 - 0.28582678745379824648e-3_f64 * t1063 * t16196 - 0.14291339372689912324e-2_f64 * t1063 * t16201 + 0.23818898954483187207e-3_f64 * t1063 * t16205 + 0.63517063878621832552e-3_f64 * t1063 * t16210 - 0.22866142996303859718e-2_f64 * t11977 * t1671 - 0.22866142996303859718e-2_f64 * t3169 * t4869 + t16218 - t16220 / 1296.0_f64 + 0.47637797908966374414e-3_f64 * t15700 * t16223 + 0.57165357490759649296e-3_f64 * t16226 * t16230;
    t16233
}
