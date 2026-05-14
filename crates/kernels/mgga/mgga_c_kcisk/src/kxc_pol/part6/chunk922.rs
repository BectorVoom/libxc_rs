//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 922/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk922<F: Float>(t12849: F, t2191: F, t26617: F, t12847: F, t1421: F, t19404: F, t2110: F, t26714: F, t26746: F, t31101: F, t31107: F, t31111: F, t31115: F, t31119: F, t31123: F, t456: F, t7828: F) -> (F,) {
    let t31127 = t12849 * t26617 * t2191;
    let t31131 = -0.19711289e-2 * t26714 - 0.43802864444444444445e-3 * t19404 - 0.98556445e-3 * t456 * t31101 - 12.0 * t2110 * t7828 + 0.59133867e-2 * t1421 * t31107 + 0.65704296666666666667e-3 * t1421 * t31111 - 0.22175200125e-2 * t1421 * t31115 + 0.22175200125e-2 * t1421 * t31119 + 0.29201909629629629629e-2 * t1421 * t31123 - 0.59133867e-2 * t12847 * t31127 + 0.1478346675e-2 * t26746;
    (t31131,)
}
