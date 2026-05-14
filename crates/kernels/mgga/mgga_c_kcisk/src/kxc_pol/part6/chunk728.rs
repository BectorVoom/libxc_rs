//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 728/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk728<F: Float>(t4998: F, t8806: F, t1773: F, t10886: F, t8810: F, t8801: F, t7208: F, t7253: F, t1769: F, t8833: F, t7219: F, t25: F, t8815: F, t8821: F, t10409: F, t8481: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23801 = t4998 * t8806;
    let t23802 = t1773 * t23801;
    let t23804 = t10886 * t8810;
    let t23805 = t1773 * t23804;
    let t23807 = t4998 * t8801;
    let t23808 = t1773 * t23807;
    let t23811 = t7208 * t7253;
    let t23814 = t8833 * t1769;
    let t23840 = t7219 * t7253;
    let t23842 = t25 * t8815;
    let t23843 = t1773 * t23842;
    let t23857 = t25 * t8821;
    let t23858 = t1773 * t23857;
    let t23872 = t10409 * t8481;
    (t23802, t23805, t23808, t23811, t23814, t23840, t23843, t23858, t23872)
}
