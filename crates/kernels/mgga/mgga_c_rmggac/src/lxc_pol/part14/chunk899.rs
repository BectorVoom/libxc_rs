//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 899/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk899<F: Float>(t2100: F, t41028: F, t2115: F, t36188: F, t36190: F, t6444: F, t8708: F, t41055: F, t793: F, t2118: F, t41048: F, t41056: F, t2103: F, t41036: F, t36175: F, t36184: F, t36192: F, t36194: F, t36201: F, t36205: F) -> (F,) {
    let t41363 = t2100 * t41028;
    let t41365 = t2115 * t41028;
    let t41367 = 0.64905642291407286545e-2 * t36188;
    let t41368 = 0.77886770749688743854e-2 * t36190;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    let t41375 = t2118 * t41048;
    let t41377 = t2100 * t41056;
    let t41378 = 0.18183107769496894486e-1 * t41377;
    let t41379 = t2103 * t41036;
    let t41380 = 0.24244143692662525982e-1 * t41379;
    let t41381 = t2118 * t41036;
    let t41383 = -0.10620923284048465071e-2 * t36175 + 0.3540307761349488357e-2 * t36184 + 0.33335697577410973225e-1 * t41363 + 0.88704377798256624947e-3 * t41365 - t41367 + t41368 + 0.74346462988339255497e-2 * t36192 + 0.88507694033737208925e-3 * t36194 + t36201 + 0.53218852008283593618e-1 * t41371 + 0.53218852008283593618e-1 * t41373 - t36205 - 0.10584045078201074568e-3 * t41375 - t41378 + t41380 + 0.56448240417072397696e-3 * t41381;
    (t41383,)
}
