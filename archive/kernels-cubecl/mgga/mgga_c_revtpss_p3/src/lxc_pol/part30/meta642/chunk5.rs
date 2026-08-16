//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2242/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2242<F: Float>(t17445: F, t7607: F, t3655: F, t8177: F, t1256: F, t29074: F, t29069: F, t29089: F, t3685: F, t17332: F, t17405: F, t2138: F, t3650: F, t3689: F, t3694: F, t3701: F, t484: F, t8184: F) -> F {
    let t104994 = t7607 * t17445 / F::cast_from(432.0_f64);
    let t104999 = t8177 * t3655;
    let t105002 = F::cast_from(0.57165357490759649296e-3_f64) * t29074 * t1256;
    let t105007 = F::cast_from(0.30488190661738479624e-2_f64) * t29069 * t1256;
    let t105014 = t29089 * t3685 / F::cast_from(162.0_f64);
    let t105017 = -t104994 + t29089 * t3689 / F::cast_from(108.0_f64) + t29089 * t3694 / F::cast_from(54.0_f64) - F::cast_from(0.95275595817932748827e-4_f64) * t104999 + t105002 + F::cast_from(0.42874018118069736972e-3_f64) * t17332 * t2138 * t484 - t105007 - F::cast_from(0.22866142996303859718e-2_f64) * t3650 * t8184 * t484 - t29089 * t3701 / F::cast_from(81.0_f64) + t105014 - t7607 * t17405 / F::cast_from(288.0_f64);
    t105017
}
