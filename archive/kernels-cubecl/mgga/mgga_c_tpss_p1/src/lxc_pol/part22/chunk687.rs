//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 687/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk687<F: Float>(t30: F, t33: F, t3274: F, t3275: F, t3273: F, t2331: F, t497: F, t489: F, t502: F, t1991: F, t3218: F, t490: F, t504: F, t2829: F, t3226: F, t493: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3276 = t3274 * t3275;
    let t3277 = t3273 * t3276;
    let t3280 = t497 * t2331;
    let t3281 = t489 * t3280;
    let t3282 = F::cast_from(1.0_f64) / t502;
    let t3288 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3282 * t3218 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t490 * t1991);
    let t3289 = F::cast_from(1.0_f64) / t504;
    let t3295 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3289 * t3226 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t493 * t2829);
    (t3277, t3280, t3281, t3282, t3288, t3289, t3295)
}
