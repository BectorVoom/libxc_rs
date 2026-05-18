//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 949/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk949<F: Float>(t11983: F, t16640: F, t16658: F, t22353: F, t22355: F, t28231: F, t28235: F, t28239: F, t28244: F, t28250: F, t28253: F, t28259: F) -> F {
    let t29782 = -F::new(0.46429444444444444443e-2) * t28231 - F::new(0.46429444444444444443e-2) * t28235 - F::new(0.58036805555555555555e-2) * t28239 + F::new(0.38691203703703703703e-2) * t28244 + F::new(0.38691203703703703703e-2) * t22353 + F::new(0.23214722222222222222e-2) * t22355 + F::new(0.69644166666666666665e-2) * t28250 + F::new(0.58036805555555555555e-2) * t28253 + t11983 - F::new(0.11607361111111111111e-2) * t16640 + F::new(0.69644166666666666665e-2) * t28259 - F::new(0.77382407407407407405e-3) * t16658;
    t29782
}
