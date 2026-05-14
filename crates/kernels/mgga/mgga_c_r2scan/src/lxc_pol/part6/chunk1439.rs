//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1439/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1439<F: Float>(t27079: F, t780: F, t7918: F, t22805: F, t22809: F, t22813: F, t22823: F, t22825: F, t27058: F, t27061: F, t27063: F, t27068: F, t27074: F, t27078: F, t495: F, t5108: F, t5109: F, t8235: F) -> (F,) {
    let t27080 = 0.87816964854445047168e-1 * t27079;
    let t27081 = t7918 * t780;
    let t27083 = -0.49390868872016336991e-1 * t22805 + 0.2037639021386884617e0 * t22809 + 0.6112917064160653851e0 * t22813 + 0.1047928639570397803e0 * t27058 - 0.34930954652346593433e-1 * t27061 - 0.34930954652346593433e-1 * t27063 - 0.34930954652346593433e-1 * t22823 - 0.34930954652346593433e-1 * t27068 - 0.39006997830244208535e0 * t5108 * t5109 * t8235 * t495 - 0.19776387377308997907e1 * t27074 + 0.27439371595564631661e-2 * t22825 - t27078 + t27080 - 0.34672886960217074253e0 * t27081;
    (t27083,)
}
