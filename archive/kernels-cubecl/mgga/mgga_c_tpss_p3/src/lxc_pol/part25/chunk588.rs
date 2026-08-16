//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 588/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk588<F: Float>(t1206: F, t520: F, t2331: F, t497: F, t489: F, t502: F, t504: F, t1170: F, t1184: F, t1186: F, t19: F, t27: F) -> (F, F, F, F, F, F, F, F) {
    let t3275 = t520 * t1206;
    let t3280 = t497 * t2331;
    let t3281 = t489 * t3280;
    let t3282 = F::cast_from(1.0_f64) / t502;
    let t3289 = F::cast_from(1.0_f64) / t504;
    let t3301 = t1170 * t1184;
    let t3304 = F::cast_from(8.0_f64) * t1170 * t1186;
    let t3305 = t19 * t27;
    (t3275, t3280, t3281, t3282, t3289, t3301, t3304, t3305)
}
