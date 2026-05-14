//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 942/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk942<F: Float>(t4811: F, t6691: F, t11237: F, t6685: F, t1869: F, t1894: F, t7069: F, t1801: F, t5062: F, t2527: F, t4797: F, t10473: F, t2474: F, t1757: F, t1899: F, t1873: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16643 = t4811 * t6691;
    let t16645 = t11237 * t6685;
    let t16646 = t1869 * t16645;
    let t16648 = t7069 * t1894;
    let t16649 = t1801 * t16648;
    let t16650 = t5062 * t16649;
    let t16651 = t1869 * t16650;
    let t16653 = t2527 * t4797;
    let t16654 = t1801 * t16653;
    let t16655 = t5062 * t16654;
    let t16656 = t1869 * t16655;
    let t16658 = t10473 * t2474;
    let t16660 = t7069 * t1757;
    let t16661 = t1899 * t16660;
    let t16662 = t1873 * t16661;
    (t16643, t16646, t16648, t16651, t16653, t16656, t16658, t16660, t16662)
}
