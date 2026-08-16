//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 805/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk805<F: Float>(t122: F, t8832: F, t649: F, t1769: F, t8825: F, t4998: F, t8806: F, t1773: F, t10886: F, t8810: F, t8801: F, t7208: F, t7253: F) -> (F, F, F, F, F, F, F) {
    let t23768 = t8832 * t122;
    let t23769 = t649 * t23768;
    let t23779 = t8825 * t1769;
    let t23801 = t4998 * t8806;
    let t23802 = t1773 * t23801;
    let t23804 = t10886 * t8810;
    let t23805 = t1773 * t23804;
    let t23807 = t4998 * t8801;
    let t23808 = t1773 * t23807;
    let t23811 = t7208 * t7253;
    (t23768, t23769, t23779, t23802, t23805, t23808, t23811)
}
