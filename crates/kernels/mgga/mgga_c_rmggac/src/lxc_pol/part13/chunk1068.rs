//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1068/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1068<F: Float>(t40121: F, t35567: F, t35577: F, t35580: F, t35584: F, t35587: F, t35591: F, t35594: F, t40123: F, t40125: F, t40136: F, t40139: F, t40143: F, t40149: F, t40154: F, t40159: F, t40164: F) -> F {
    let t43288 = F::new(0.11918087970123395032e-3) * t40121;
    let t43302 = -F::new(0.11918087970123395032e-3) * t35567 - F::new(0.19863479950205658386e-4) * t35577 - F::new(0.39726959900411316772e-4) * t35580 + t43288 + F::new(0.49658699875514145966e-4) * t40123 + F::new(0.49658699875514145966e-4) * t40125 - F::new(0.11708147441822390596e1) * t35584 + F::new(0.17562221162733585894e1) * t35587 + F::new(0.5854073720911195298e0) * t35591 - F::new(0.20455996240684006298e-1) * t40136 + F::new(0.40911992481368012596e-1) * t40139 - F::new(0.212822999466489197e-4) * t40143 - F::new(0.425645998932978394e-4) * t40149 - F::new(0.3405167991463827152e-4) * t40154 + F::new(0.10215503974391481456e-3) * t40159 - F::new(0.10215503974391481456e-3) * t40164 + F::new(0.79828278012425390427e-1) * t35594;
    t43302
}
