//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1107/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1107<F: Float>(t10068: F, t10074: F, t10080: F, t10086: F, t10092: F, t10097: F, t10487: F, t10496: F, t37031: F, t42313: F, t9614: F, t10116: F, t10196: F, t37039: F, t42320: F, t42322: F, t42323: F, t42324: F, t7909: F, t9107: F, t9631: F, t9636: F) -> (F, F) {
    let t48074 = -t9614 + t10487 + t10068 + t10074 - t10080 - t42313 - t37031 - t10496 - t10086 + t10092 + t10097;
    let t48080 = t7909 - t9631 - t42320 + F::new(0.25538759935978703639e-4) * t9107 + t42322 - t42323 - t42324 + t9636 + t10116 + t10196 + t37039;
    (t48074, t48080)
}
