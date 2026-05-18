//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 576/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk576<F: Float>(t4825: F, t4824: F, t1689: F, t1692: F, t172: F, t342: F, t569: F, t142: F, t673: F, t1224: F, t1636: F, t10: F, t1797: F) -> (F, F, F, F, F, F, F, F) {
    let t4826 = F::new(1.0) / t4825;
    let t4827 = t4824 * t4826;
    let t4830 = t1689 * t1692;
    let t4834 = t342 * t172 * t569;
    let t4835 = F::new(0.23744444444444444444e-1) * t4834;
    let t4836 = t142 * t673;
    let t4838 = t1224 * t4836 * t1636;
    let t4840 = t10 * t1797;
    (t4826, t4827, t4830, t4834, t4835, t4836, t4838, t4840)
}
