//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 840/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk840<F: Float>(t41438: F, t874: F, t8794: F, t39700: F, t797: F, t40897: F, t5271: F, t40920: F, t5162: F, t38568: F, t4669: F, t1587: F, t2064: F) -> (F, F, F, F, F, F, F) {
    let t41439 = F::new(0.15965655602485078085e0) * t41438;
    let t41483 = t874 * t8794;
    let t41523 = t797 * t39700;
    let t41524 = F::new(0.23948483403727617128e0) * t41523;
    let t41531 = t5271 * t40897;
    let t41532 = F::new(0.47896966807455234256e0) * t41531;
    let t41534 = t5162 * t40920;
    let t41535 = F::new(0.95793933614910468512e0) * t41534;
    let t41536 = t4669 * t38568;
    let t41537 = F::new(0.23948483403727617128e0) * t41536;
    let t41548 = t2064 * t1587;
    (t41439, t41483, t41524, t41532, t41535, t41537, t41548)
}
