//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 804/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk804<F: Float>(t11252: F, t1421: F, t22412: F, t22414: F, t28822: F, t28826: F, t28830: F, t28834: F, t28837: F, t28841: F, t28847: F, t28881: F, t28909: F, t28948: F, t1801: F, t1873: F) -> (F, F) {
    let t28950 = 0.39422577999999999999e-2 * t1421 * t28822 + 0.39422577999999999999e-2 * t1421 * t28826 + 0.1478346675e-2 * t1421 * t28830 - 0.59133867e-2 * t1421 * t28834 - 0.39422577999999999999e-2 * t1421 * t28837 + 0.295669335e-2 * t1421 * t28841 + 0.39422578e-2 * t22412 - 0.26281718666666666667e-2 * t22414 + t11252 - 0.4435040025e-2 * t1421 * t28847 + t28881 + t28909 + t28948;
    let t28951 = t1801 * t28950;
    let t28952 = t1873 * t28951;
    (t28950, t28952)
}
