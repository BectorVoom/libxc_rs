//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 699/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk699<F: Float>(t3478: F, t3356: F, t1175: F, t1179: F, t1178: F, t444: F) -> (F, F, F, F) {
    let t3479 = F::new(1.0) / t3478;
    let t3483 = F::cast_from(0.12361111111111111111e-1_f64) * t3356;
    let t3491 = t1175 * t1179;
    let t3494 = t1178 * t444;
    let t3495 = F::new(1.0) / t3494;
    (t3479, t3483, t3491, t3495)
}
