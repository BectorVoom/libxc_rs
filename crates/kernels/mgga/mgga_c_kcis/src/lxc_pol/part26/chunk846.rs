//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 846/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk846<F: Float>(t11951: F, t1650: F, t12048: F, t167: F, t1444: F, t2622: F, t1445: F, t5654: F, t12065: F, t3754: F, t822: F, t5851: F, t733: F) -> (F, F, F, F, F, F, F) {
    let t17098 = t11951 * t1650;
    let t17100 = t12048 * t167;
    let t17102 = t2622 * t1444;
    let t17103 = t17102 * t167;
    let t17137 = F::new(0.47822877300252710492e-1) * t1445 * t5654;
    let t17143 = F::new(0.62154466893555682512e-3) * t12065 * t5654;
    let t17146 = t822 * t3754;
    let t17150 = F::new(0.18736e-1) * t733 * t5851;
    (t17098, t17100, t17103, t17137, t17143, t17146, t17150)
}
