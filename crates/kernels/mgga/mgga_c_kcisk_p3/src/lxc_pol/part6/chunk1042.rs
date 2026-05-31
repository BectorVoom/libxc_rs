//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1042/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1042<F: Float>(t5927: F, t7764: F, t5926: F, t13220: F, t13221: F, t30153: F, t12849: F, t2191: F, t26617: F, t12847: F, t1421: F, t19404: F, t2110: F, t26714: F, t26746: F, t31101: F, t31107: F, t31111: F, t31115: F, t456: F, t7828: F) -> (F, F) {
    let t31118 = t5927 * t7764;
    let t31119 = t5926 * t31118;
    let t31123 = t13220 * t13221 * t30153;
    let t31127 = t12849 * t26617 * t2191;
    let t31131 = -F::cast_from(0.19711289e-2_f64) * t26714 - F::cast_from(0.43802864444444444445e-3_f64) * t19404 - F::cast_from(0.98556445e-3_f64) * t456 * t31101 - F::cast_from(12.0_f64) * t2110 * t7828 + F::cast_from(0.59133867e-2_f64) * t1421 * t31107 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t31111 - F::cast_from(0.22175200125e-2_f64) * t1421 * t31115 + F::cast_from(0.22175200125e-2_f64) * t1421 * t31119 + F::cast_from(0.29201909629629629629e-2_f64) * t1421 * t31123 - F::cast_from(0.59133867e-2_f64) * t12847 * t31127 + F::cast_from(0.1478346675e-2_f64) * t26746;
    (t31118, t31131)
}
