//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 823/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk823<F: Float>(t40479: F, t1982: F, t7428: F, t8688: F, t1627: F, t2064: F, t3928: F, t34884: F, t8668: F, t8831: F, t8836: F, t8843: F) -> (F, F, F, F, F, F, F) {
    let t40480 = F::new(0.19863479950205658386e-4) * t40479;
    let t40505 = t8688 * t7428 * t1982;
    let t40506 = F::new(0.19863479950205658386e-4) * t40505;
    let t40516 = t3928 * t2064 * t1627;
    let t40558 = t34884 * t8668;
    let t40559 = F::new(0.24829349937757072982e-4) * t40558;
    let t40560 = t34884 * t8831;
    let t40561 = F::new(0.74488049813271218946e-4) * t40560;
    let t40562 = t34884 * t8836;
    let t40563 = F::new(0.74488049813271218946e-4) * t40562;
    let t40564 = t34884 * t8843;
    (t40480, t40506, t40516, t40559, t40561, t40563, t40564)
}
