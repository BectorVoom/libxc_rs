//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 487/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk487<F: Float>(t1691: F, t670: F, t604: F, t667: F, t172: F, t342: F, t569: F, t142: F, t673: F, t10: F, t1797: F, t1704: F, t617: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4822 = F::cast_from(1.0_f64) / t1691 / t670;
    let t4823 = t604 * t4822;
    let t4825 = t667 * t667;
    let t4826 = F::cast_from(1.0_f64) / t4825;
    let t4834 = t342 * t172 * t569;
    let t4835 = F::cast_from(0.23744444444444444444e-1_f64) * t4834;
    let t4836 = t142 * t673;
    let t4840 = t10 * t1797;
    let t4856 = t1704 * t617;
    (t4822, t4823, t4825, t4826, t4834, t4835, t4836, t4840, t4856)
}
