//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1192/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1192<F: Float>(t1674: F, t1713: F, t8034: F, t10040: F, t1427: F, t1680: F, t2166: F, t24753: F, t24893: F, t32278: F, t36684: F, t36686: F, t36689: F, t38559: F, t38563: F, t5645: F, t567: F, t7297: F, t8040: F, t8372: F, t9448: F, t9460: F, t9469: F) -> F {
    let t40992 = t1674 * t8034 * t1713;
    let t41000 = -t10040 * t2166 * t567 + F::new(12.0) * t1427 * t36686 * t8372 - F::new(2.0) * t1680 * t567 * t9448 - F::new(3.0) * t24753 * t7297 * t8040 - F::new(6.0) * t24893 * t8040 * t8372 + F::new(6.0) * t32278 * t567 * t9469 + F::new(12.0) * t38559 * t7297 * t9460 - F::new(6.0) * t38563 * t7297 * t8040 + F::new(12.0) * t5645 * t8034 * t8372 - t36684 + t36689 + F::new(6.0) * t40992;
    t41000
}
