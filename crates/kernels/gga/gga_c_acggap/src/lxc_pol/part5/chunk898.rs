//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 898/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk898<F: Float>(t13326: F, t150: F, t164: F, t177: F, t968: F, t977: F, t151: F, t161: F, t7510: F, t3171: F, t3372: F, t171: F, t368: F) -> (F, F, F, F, F) {
    let t13330 = F::new(0.21437009059034868486e-3) * t13326 * t150 * t164 * t177;
    let t13332 = t977 * t968;
    let t13337 = F::new(0.28974367305964659283e0) * t151 * t161 * t7510 * t177;
    let t13344 = t3372 * t3171;
    let t13364 = t171 * t368;
    (t13330, t13332, t13337, t13344, t13364)
}
