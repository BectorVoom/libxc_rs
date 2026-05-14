//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1242/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1242<F: Float>(t3353: F, t8865: F, t3316: F, t8854: F, t20843: F, t4114: F, t4140: F, t6569: F, t20703: F, t20706: F, t20770: F, t24556: F, t24559: F, t24562: F, t251: F, t28853: F, t28856: F, t28859: F) -> (F, F, F, F, F) {
    let t29040 = 4.0 * t8865 * t3353;
    let t29042 = 2.0 * t3316 * t8854;
    let t29044 = 2.0 * t20843 * t4114;
    let t29046 = 1.0 * t6569 * t4140;
    let t29057 = 0.621814e-1 * (t20770 - 0.11080740740740740741e0 * t20703 + 0.23744444444444444444e-1 * t20706 - 0.11080740740740740741e0 * t24556 + 0.94977777777777777776e-1 * t24559 - 0.35616666666666666666e-1 * t24562 + 0.23744444444444444444e-1 * t28859 - 0.35616666666666666666e-1 * t28853 + 0.53425e-1 * t28856) * t251;
    (t29040, t29042, t29044, t29046, t29057)
}
