//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 572/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk572<F: Float>(t3523: F, t3542: F, t1196: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t459: F) -> (F, F, F, F) {
    let t3543 = t3542 * t3523;
    let t3545 = F::cast_from(0.17315859105681463759e2_f64) * t1196 * t3543;
    let t3546 = F::cast_from(0.11111111111111111111e-1_f64) * t3356;
    let t3551 = t3546 - F::cast_from(0.55555555555555555556e-2_f64) * t3358 - F::cast_from(0.55555555555555555555e-2_f64) * t3365 + F::cast_from(0.16666666666666666667e-1_f64) * t3370 + F::cast_from(0.83333333333333333333e-2_f64) * t3374;
    let t3552 = t3551 * t459;
    (t3543, t3545, t3551, t3552)
}
