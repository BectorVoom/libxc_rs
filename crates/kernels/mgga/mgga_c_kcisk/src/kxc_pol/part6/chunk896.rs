//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 896/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk896<F: Float>(t11252: F, t1421: F, t22412: F, t22414: F, t28822: F, t28826: F, t28830: F, t28834: F, t28837: F, t28841: F, t28847: F, t28881: F, t28909: F, t28948: F) -> F {
    let t28950 = F::cast_from(0.39422577999999999999e-2_f64) * t1421 * t28822 + F::cast_from(0.39422577999999999999e-2_f64) * t1421 * t28826 + F::cast_from(0.1478346675e-2_f64) * t1421 * t28830 - F::new(0.59133867e-2) * t1421 * t28834 - F::cast_from(0.39422577999999999999e-2_f64) * t1421 * t28837 + F::cast_from(0.295669335e-2_f64) * t1421 * t28841 + F::new(0.39422578e-2) * t22412 - F::cast_from(0.26281718666666666667e-2_f64) * t22414 + t11252 - F::cast_from(0.4435040025e-2_f64) * t1421 * t28847 + t28881 + t28909 + t28948;
    t28950
}
