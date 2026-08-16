//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 445/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk445<F: Float>(t2154: F, t314: F, t299: F, t911: F, t107: F, t2067: F, t2070: F, t2073: F, t2077: F, t2081: F, t2087: F, t2092: F, t2096: F, t2098: F, t2103: F, t2104: F, t2110: F, t315: F, t317: F, t327: F, t775: F, t776: F, t802: F, t813: F, t819: F, t833: F) -> (F, F, F) {
    let t2155 = t2154 * t314;
    let t2158 = t911 * t299;
    let t2159 = t107 * t2158;
    let t2162 = -F::cast_from(0.71500979903700853338e0_f64) * t2067 * t776 + F::cast_from(0.71500979903700853338e0_f64) * t2070 * t317 - F::cast_from(0.35750489951850426669e0_f64) * t775 * t2073 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t2077 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t2081 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t2092 - F::cast_from(0.10725146985555128001e1_f64) * t2096 * t2098 + F::cast_from(0.71500979903700853338e0_f64) * t2103 * t2104 - F::cast_from(0.30674340763136599742e1_f64) * t802 * t819 - F::cast_from(0.11502877786176224903e1_f64) * t2110 * t327 + F::cast_from(0.35750489951850426669e0_f64) * t2155 * t317 - F::cast_from(0.79445533226334281487e-1_f64) * t315 * t2159;
    (t2158, t2159, t2162)
}
