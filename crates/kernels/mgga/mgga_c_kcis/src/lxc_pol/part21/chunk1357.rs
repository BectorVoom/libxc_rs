//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1357/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1357<F: Float>(t95903: F, t26960: F, t28102: F, t7775: F, t7796: F, t8087: F, t92830: F, t93082: F, t95895: F, t95906: F, t97010: F, t97015: F, t97019: F, t97026: F, t97028: F, t97030: F) -> F {
    let t97031 = F::new(0.15476481481481481481e-2) * t95903;
    let t97033 = -F::new(0.24734586805555555556e-3) * t92830 * t8087 + F::new(0.23214722222222222222e-2) * t95895 - F::new(0.18534722222222222222e-2) * t97010 * t7796 - F::new(0.18534722222222222222e-2) * t97010 * t7775 - F::new(0.24734586805555555556e-3) * t97015 * t7775 - F::new(0.23168402777777777778e-3) * t26960 * t97019 - F::new(0.82448622685185185185e-4) * t93082 * t28102 - t97026 - t97028 - t97030 - t97031 + F::new(0.61905925925925925925e-2) * t95906;
    t97033
}
