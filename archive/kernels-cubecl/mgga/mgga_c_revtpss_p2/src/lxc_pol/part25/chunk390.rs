//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 390/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk390<F: Float>(t1419: F, t225: F, t561: F, t213: F, t555: F, t560: F) -> (F, F, F, F, F) {
    let t1420 = t1419 * t225;
    let t1421 = t1420 * t561;
    let t1424 = t213 * t555;
    let t1425 = t560 * t560;
    let t1426 = F::cast_from(1.0_f64) / t1425;
    (t1420, t1421, t1424, t1425, t1426)
}
