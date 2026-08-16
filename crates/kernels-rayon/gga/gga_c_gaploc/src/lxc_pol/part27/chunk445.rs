//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 445/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk445(t2154: f64, t314: f64, t299: f64, t911: f64, t107: f64, t2067: f64, t2070: f64, t2073: f64, t2077: f64, t2081: f64, t2087: f64, t2092: f64, t2096: f64, t2098: f64, t2103: f64, t2104: f64, t2110: f64, t315: f64, t317: f64, t327: f64, t775: f64, t776: f64, t802: f64, t813: f64, t819: f64, t833: f64) -> (f64, f64, f64) {
    let t2155 = t2154 * t314;
    let t2158 = t911 * t299;
    let t2159 = t107 * t2158;
    let t2162 = -0.71500979903700853338e0_f64 * t2067 * t776 + 0.71500979903700853338e0_f64 * t2070 * t317 - 0.35750489951850426669e0_f64 * t775 * t2073 - 0.46011511144704899612e1_f64 * t813 * t2077 + 0.11502877786176224903e2_f64 * t833 * t2081 - 0.69017266717057349418e1_f64 * t2087 * t2092 - 0.10725146985555128001e1_f64 * t2096 * t2098 + 0.71500979903700853338e0_f64 * t2103 * t2104 - 0.30674340763136599742e1_f64 * t802 * t819 - 0.11502877786176224903e1_f64 * t2110 * t327 + 0.35750489951850426669e0_f64 * t2155 * t317 - 0.79445533226334281487e-1_f64 * t315 * t2159;
    (t2158, t2159, t2162)
}
