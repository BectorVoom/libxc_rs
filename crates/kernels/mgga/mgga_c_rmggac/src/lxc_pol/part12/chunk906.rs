//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 906/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk906<F: Float>(t40920: F, t5162: F, t38568: F, t4669: F, t27041: F, t38798: F, t25820: F, t38801: F, t25877: F, t38792: F, t38795: F, t1587: F, t2064: F, t793: F, t798: F, t8975: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41534 = t5162 * t40920;
    let t41535 = 0.95793933614910468512e0 * t41534;
    let t41536 = t4669 * t38568;
    let t41537 = 0.23948483403727617128e0 * t41536;
    let t41538 = t27041 * t38798;
    let t41540 = t25820 * t38801;
    let t41542 = t25877 * t38792;
    let t41544 = t25820 * t38795;
    let t41548 = t2064 * t1587;
    let t41549 = t793 * t41548;
    let t41550 = 0.15965655602485078085e0 * t41549;
    let t41551 = t8975 * t798;
    (t41535, t41537, t41538, t41540, t41542, t41544, t41548, t41550, t41551)
}
