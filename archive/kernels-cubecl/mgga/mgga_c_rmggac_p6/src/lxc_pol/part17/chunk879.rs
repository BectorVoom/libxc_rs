//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 879/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk879<F: Float>(t38530: F, t8447: F, t2318: F, t34976: F, t39437: F, t9145: F, t3369: F, t34975: F, t559: F, t8455: F, t16156: F, t9964: F) -> (F, F, F, F) {
    let t44773 = t38530 * t8447;
    let t44777 = t39437 * t34976 * t2318 * t9145;
    let t44781 = t34975 * t3369 * t559 * t8455;
    let t44784 = t16156 * t9964;
    (t44773, t44777, t44781, t44784)
}
