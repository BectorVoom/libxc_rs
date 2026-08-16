//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 891/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk891(t28873: f64, t706: f64, t11400: f64, t1421: f64, t16879: f64, t22469: f64, t22512: f64, t22524: f64, t28532: f64, t28852: f64, t28856: f64, t28860: f64, t28865: f64, t28869: f64, t456: f64, t604: f64) -> f64 {
    let t28874 = t706 * t28873;
    let t28881 = -0.4435040025e-2_f64 * t1421 * t28852 + 0.887008005e-2_f64 * t1421 * t28856 - 0.59133867e-2_f64 * t456 * t28860 + 0.1478346675e-2_f64 * t22469 - 0.59133867e-2_f64 * t11400 * t28865 - 0.98556445e-3_f64 * t456 * t28869 - 0.19711289e-2_f64 * t22512 + 0.1478346675e-2_f64 * t456 * t28874 + 0.295669335e-2_f64 * t22524 - 4.0_f64 * t604 * t28532 + 0.65704296666666666665e-3_f64 * t16879;
    t28881
}
