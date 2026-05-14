//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1127/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1127<F: Float>(t11611: F, t12658: F, t33091: F, t33093: F, t33094: F, t33095: F, t33096: F, t33097: F, t33098: F, t33099: F, t33100: F, t33101: F, t36089: F, t36090: F, t38846: F, t38848: F, t7: F) -> (F,) {
    let tv4rho2sigma26 = t33091 + t11611 + t33093 - t33094 + t7 * (t38846 + t38848) + t33095 - t33096 + t12658 + t33097 + t33098 - t33099 - t33100 + t33101 - t36089 - t36090;
    (tv4rho2sigma26,)
}
