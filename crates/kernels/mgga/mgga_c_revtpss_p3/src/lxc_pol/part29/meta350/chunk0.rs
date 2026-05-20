//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1279/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1279<F: Float>(t159: F, t3617: F, t409: F, t416: F, t406: F, t12295: F, t11335: F, t281: F, t414: F, t1126: F, t3383: F, t1160: F, t3444: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12305 = t159 * t3617;
    let t12327 = F::new(1.0) / t409 / t416 / F::new(4.0);
    let t12331 = F::new(1.0)/pow_3_2::<F>(t406);
    let t12349 = F::cast_from(0.93011851851851851854e0_f64) * t12295;
    let t12351 = t281 * t11335 * t414;
    let t12352 = F::cast_from(0.36514074074074074075e0_f64) * t12351;
    let t12361 = t1126 * t3383;
    let t12367 = F::cast_from(0.28842592592592592592e-1_f64) * t12295;
    let t12382 = F::cast_from(0.55403703703703703703e-1_f64) * t12295;
    let t12397 = F::cast_from(0.53272592592592592592e-1_f64) * t12295;
    let t12418 = t3444 * t1160;
    (t12305, t12327, t12331, t12349, t12351, t12352, t12361, t12367, t12382, t12397, t12418)
}
