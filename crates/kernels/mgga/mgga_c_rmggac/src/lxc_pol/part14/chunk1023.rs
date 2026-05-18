//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1023/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1023<F: Float>(t2100: F, t41028: F, t2115: F, t36188: F, t36190: F, t6444: F, t8708: F, t41055: F, t793: F, t2118: F, t41048: F, t41056: F) -> (F, F, F, F, F, F, F, F) {
    let t41363 = t2100 * t41028;
    let t41365 = t2115 * t41028;
    let t41367 = F::new(0.64905642291407286545e-2) * t36188;
    let t41368 = F::new(0.77886770749688743854e-2) * t36190;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    let t41375 = t2118 * t41048;
    let t41377 = t2100 * t41056;
    (t41363, t41365, t41367, t41368, t41371, t41373, t41375, t41377)
}
