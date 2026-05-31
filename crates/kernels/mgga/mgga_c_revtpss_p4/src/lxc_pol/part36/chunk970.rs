//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 970/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk970<F: Float>(t13652: F, t13654: F, t9415: F, t9421: F, t9427: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F) -> (F, F, F) {
    let t22925 = F::cast_from(0.51947577317044391276e2_f64) * t13652;
    let t22926 = F::cast_from(24.0_f64) * t13654;
    let t22927 = -t9415 + t9421 - t9427 + t9546 + t9514 - t9517 - t9521 + t9569 - t9574 - t9577 - t22925 - t22926;
    (t22925, t22926, t22927)
}
