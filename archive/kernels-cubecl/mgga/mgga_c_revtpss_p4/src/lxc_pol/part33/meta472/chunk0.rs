//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1720/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1720<F: Float>(t9422: F, t9559: F, t9566: F, t9570: F, t9578: F, t13643: F, t9421: F, t9427: F, t9429: F, t9514: F, t9517: F, t9521: F, t9546: F, t9569: F, t9574: F, t9577: F) -> (F, F, F, F, F, F) {
    let t22205 = F::cast_from(0.11696447245269292414e1_f64) * t9422;
    let t22206 = F::cast_from(20.0_f64) * t9559;
    let t22207 = F::cast_from(0.24415263074675393405e-3_f64) * t9566;
    let t22208 = F::cast_from(32.0_f64) * t9570;
    let t22209 = F::cast_from(12.0_f64) * t9578;
    let t22210 = t9421 + t22205 - t9427 + t9429 + t9546 + t22206 + t9514 - t13643 + t22207 - t9517 - t9521 + t9569 + t22208 - t9574 - t9577 + t22209;
    (t22205, t22206, t22207, t22208, t22209, t22210)
}
