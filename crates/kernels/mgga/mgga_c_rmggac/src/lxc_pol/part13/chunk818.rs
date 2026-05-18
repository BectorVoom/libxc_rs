//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 818/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk818<F: Float>(t17859: F, t7251: F, t7738: F, t7376: F, t7746: F, t1987: F, t38472: F, t3924: F, t623: F, t7275: F, t34761: F, t8447: F) -> (F, F, F, F, F, F, F) {
    let t38485 = t17859 * t7251;
    let t38487 = t17859 * t7738;
    let t38489 = t17859 * t7376;
    let t38491 = t17859 * t7746;
    let t38493 = t38472 * t1987;
    let t38495 = t623 * t3924;
    let t38496 = t38495 * t7275;
    let t38498 = t34761 * t8447;
    (t38485, t38487, t38489, t38491, t38493, t38496, t38498)
}
