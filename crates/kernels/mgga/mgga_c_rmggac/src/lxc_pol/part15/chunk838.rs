//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 838/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk838<F: Float>(t41340: F, t3814: F, t40920: F, t2115: F, t41056: F, t2103: F, t41032: F, t2100: F, t41028: F, t6444: F, t8708: F, t41055: F, t793: F) -> (F, F, F, F, F, F, F, F) {
    let t41341 = F::new(0.10620923284048465071e-1) * t41340;
    let t41342 = t3814 * t40920;
    let t41347 = t2115 * t41056;
    let t41348 = F::new(0.4838420607177634088e-3) * t41347;
    let t41355 = t2103 * t41032;
    let t41363 = t2100 * t41028;
    let t41365 = t2115 * t41028;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    (t41341, t41342, t41348, t41355, t41363, t41365, t41371, t41373)
}
