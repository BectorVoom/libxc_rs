//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1042/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1042(t5927: f64, t7764: f64, t5926: f64, t13220: f64, t13221: f64, t30153: f64, t12849: f64, t2191: f64, t26617: f64, t12847: f64, t1421: f64, t19404: f64, t2110: f64, t26714: f64, t26746: f64, t31101: f64, t31107: f64, t31111: f64, t31115: f64, t456: f64, t7828: f64) -> (f64, f64) {
    let t31118 = t5927 * t7764;
    let t31119 = t5926 * t31118;
    let t31123 = t13220 * t13221 * t30153;
    let t31127 = t12849 * t26617 * t2191;
    let t31131 = -0.19711289e-2_f64 * t26714 - 0.43802864444444444445e-3_f64 * t19404 - 0.98556445e-3_f64 * t456 * t31101 - 12.0_f64 * t2110 * t7828 + 0.59133867e-2_f64 * t1421 * t31107 + 0.65704296666666666667e-3_f64 * t1421 * t31111 - 0.22175200125e-2_f64 * t1421 * t31115 + 0.22175200125e-2_f64 * t1421 * t31119 + 0.29201909629629629629e-2_f64 * t1421 * t31123 - 0.59133867e-2_f64 * t12847 * t31127 + 0.1478346675e-2_f64 * t26746;
    (t31118, t31131)
}
