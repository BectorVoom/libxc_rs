//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 920/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk920<F: Float>(t28241: F, t28273: F, t28323: F, t28728: F, t28754: F, t28785: F, t28955: F, t29346: F, t752: F, t24081: F, t2594: F, t17775: F, t8968: F) -> (F, F, F) {
    let t29349 = t28241 + t28273 + t28323 + t28728 + t28754 + t28785 + t28955 + t29346;
    let t29350 = t29349 * t752;
    let t29352 = F::new(3.0) * t24081 * t2594;
    let t29354 = F::new(6.0) * t17775 * t8968;
    (t29350, t29352, t29354)
}
