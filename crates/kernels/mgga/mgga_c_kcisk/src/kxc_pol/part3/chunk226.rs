//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 226/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk226<F: Float>(t151: F, t167: F, t175: F, t60: F, t852: F, t945: F, t955: F, t972: F) -> F {
    let t974 = -F::cast_from(0.11713266981940447749e-2_f64) * t167 * t151 - F::cast_from(0.23426533963880895498e-2_f64) * t945 * t955 - t852 * t175 - t60 * t972;
    t974
}
