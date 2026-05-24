//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 891/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk891<F: Float>(t28873: F, t706: F, t11400: F, t1421: F, t16879: F, t22469: F, t22512: F, t22524: F, t28532: F, t28852: F, t28856: F, t28860: F, t28865: F, t28869: F, t456: F, t604: F) -> F {
    let t28874 = t706 * t28873;
    let t28881 = -F::cast_from(0.4435040025e-2_f64) * t1421 * t28852 + F::cast_from(0.887008005e-2_f64) * t1421 * t28856 - F::new(0.59133867e-2) * t456 * t28860 + F::cast_from(0.1478346675e-2_f64) * t22469 - F::new(0.59133867e-2) * t11400 * t28865 - F::new(0.98556445e-3) * t456 * t28869 - F::new(0.19711289e-2) * t22512 + F::cast_from(0.1478346675e-2_f64) * t456 * t28874 + F::cast_from(0.295669335e-2_f64) * t22524 - F::new(4.0) * t604 * t28532 + F::cast_from(0.65704296666666666665e-3_f64) * t16879;
    t28881
}
