//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 859/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk859<F: Float>(t40897: F, t5271: F, t40920: F, t5162: F, t38568: F, t4669: F, t27041: F, t38798: F, t25820: F, t38801: F, t25877: F, t38792: F, t38795: F, t1587: F, t2064: F, t793: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41531 = t5271 * t40897;
    let t41534 = t5162 * t40920;
    let t41536 = t4669 * t38568;
    let t41538 = t27041 * t38798;
    let t41540 = t25820 * t38801;
    let t41542 = t25877 * t38792;
    let t41544 = t25820 * t38795;
    let t41548 = t2064 * t1587;
    let t41549 = t793 * t41548;
    (t41531, t41534, t41536, t41538, t41540, t41542, t41544, t41548, t41549)
}
