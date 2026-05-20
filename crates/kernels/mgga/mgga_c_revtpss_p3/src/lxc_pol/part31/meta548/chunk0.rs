//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1941/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1941<F: Float>(t1089: F, t29759: F, t1972: F, t6317: F, t1675: F, t25538: F, t27448: F, t27460: F, t27462: F, t27471: F, t27489: F, t375: F, t6285: F, t6289: F, t6293: F, t6323: F, t6327: F, t7111: F, t7132: F) -> (F, F, F) {
    let t29760 = t29759 * t1089;
    let t29779 = t6317 * t1972;
    let t29782 = F::cast_from(0.57165357490759649296e-3_f64) * t27448 + t27460 / F::new(432.0) + F::cast_from(0.57165357490759649296e-3_f64) * t27462 - F::cast_from(0.57165357490759649296e-3_f64) * t27471 - t7111 * t6285 / F::new(144.0) + t7111 * t6289 / F::new(288.0) + t7111 * t6293 / F::new(216.0) + F::cast_from(0.28582678745379824648e-3_f64) * t7132 * t6323 + F::cast_from(0.47637797908966374413e-3_f64) * t7132 * t6327 + F::cast_from(0.57165357490759649296e-3_f64) * t27489 * t1675 - t25538 + F::cast_from(0.42874018118069736972e-3_f64) * t29779 * t375;
    (t29760, t29779, t29782)
}
