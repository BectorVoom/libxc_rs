//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 902/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk902<F: Float>(t3523: F, t6555: F, t1196: F, t3546: F, t5044: F, t6423: F, t6427: F, t6431: F, t459: F) -> (F, F, F, F) {
    let t6556 = t6555 * t3523;
    let t6558 = F::new(0.17315859105681463759e2) * t1196 * t6556;
    let t6563 = t3546 - F::new(0.55555555555555555556e-2) * t5044 - F::new(0.55555555555555555555e-2) * t6423 + F::new(0.16666666666666666667e-1) * t6427 + F::new(0.83333333333333333333e-2) * t6431;
    let t6564 = t6563 * t459;
    (t6556, t6558, t6563, t6564)
}
