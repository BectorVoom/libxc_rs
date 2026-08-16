//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 853/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk853<F: Float>(t10335: F, t344: F, t221: F, t339: F, t2393: F, t374: F, t376: F, t370: F, t3036: F, t67: F, t3067: F, t3186: F) -> (F, F, F, F, F, F) {
    let t10336 = t10335 * t344;
    let t10337 = t221 * t10336;
    let t10339 = F::cast_from(0.3086419753086419753e-3_f64) * t339 * t10337;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / F::cast_from(10368.0_f64);
    let t10383 = t221 * t10335;
    let t10385 = F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t339 * t10383;
    let t10401 = t3036 * t67;
    let t10402 = t3067 * t10401;
    let t10403 = t3186 * t10402;
    (t10339, t10377, t10385, t10401, t10402, t10403)
}
