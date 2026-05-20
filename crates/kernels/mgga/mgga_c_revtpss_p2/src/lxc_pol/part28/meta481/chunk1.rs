//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1826/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1826<F: Float>(t1068: F, t25543: F, t25551: F, t25554: F, t25557: F, t25560: F, t25561: F, t25564: F, t25566: F, t25569: F, t25577: F, t25580: F, t3101: F, t3120: F, t3177: F, t3184: F, t3238: F, t3248: F, t3255: F, t375: F, t7111: F, t7132: F) -> F {
    let t25585 = t25543 / F::new(432.0) + t7111 * t3248 / F::new(288.0) + t7111 * t3255 / F::new(216.0) - t7111 * t3238 / F::new(144.0) + F::cast_from(0.3811023832717309953e-3_f64) * t25551 + F::cast_from(0.14481890564325777821e-1_f64) * t25554 * t375 - F::cast_from(0.30488190661738479624e-2_f64) * t25557 - t25560 - F::cast_from(0.45732285992607719436e-2_f64) * t25561 * t375 + F::cast_from(0.57165357490759649296e-3_f64) * t25564 + F::cast_from(0.42874018118069736972e-3_f64) * t25566 * t375 + F::cast_from(0.57165357490759649296e-3_f64) * t25569 * t1068 + F::cast_from(0.47637797908966374413e-3_f64) * t7132 * t3184 + F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t3177 - F::cast_from(0.30488190661738479624e-2_f64) * t25577 * t1068 - F::cast_from(0.85748036236139473944e-3_f64) * t25580 * t3120 - F::cast_from(0.57165357490759649296e-3_f64) * t7132 * t3101;
    t25585
}
