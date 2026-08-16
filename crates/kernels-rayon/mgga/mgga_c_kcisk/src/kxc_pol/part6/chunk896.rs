//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 896/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk896(t11252: f64, t1421: f64, t22412: f64, t22414: f64, t28822: f64, t28826: f64, t28830: f64, t28834: f64, t28837: f64, t28841: f64, t28847: f64, t28881: f64, t28909: f64, t28948: f64) -> f64 {
    let t28950 = 0.39422577999999999999e-2_f64 * t1421 * t28822 + 0.39422577999999999999e-2_f64 * t1421 * t28826 + 0.1478346675e-2_f64 * t1421 * t28830 - 0.59133867e-2_f64 * t1421 * t28834 - 0.39422577999999999999e-2_f64 * t1421 * t28837 + 0.295669335e-2_f64 * t1421 * t28841 + 0.39422578e-2_f64 * t22412 - 0.26281718666666666667e-2_f64 * t22414 + t11252 - 0.4435040025e-2_f64 * t1421 * t28847 + t28881 + t28909 + t28948;
    t28950
}
