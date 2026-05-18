//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 535/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk535<F: Float>(t2323: F, t2892: F, t97: F, t2453: F, t2454: F, t990: F, t1248: F, t1217: F, t413: F, t298: F, t302: F, t994: F, rho1: F) -> (F, F, F, F, F, F, F, F) {
    let t2894 = t97 * t2323 * t2892;
    let t2895 = F::new(6.0) * t2894;
    let t2896 = F::new(2.0) * t2453;
    let t2897 = F::new(8.0) * t2454;
    let t2900 = t990 * t990;
    let t2901 = t1248 * t2900;
    let t2904 = t413 + t1217;
    let t2905 = t298 * t2904;
    let t2910 = F::new(1.0) / t302 / t994 / rho1;
    (t2895, t2896, t2897, t2900, t2901, t2904, t2905, t2910)
}
