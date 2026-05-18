//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 856/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk856<F: Float>(t170: F, t7760: F, t1651: F, t2769: F, t5411: F, t5413: F, t5433: F, t5437: F, t5441: F, t5444: F, t596: F, t7745: F, t7751: F, t7753: F, t7756: F) -> F {
    let t7761 = t7760 * t170;
    let t7764 = -F::new(0.10843581300301739842e-1) * t7745 + F::new(0.5848223622634646207e0) * t5411 + F::new(0.11696447245269292414e1) * t5413 + t5433 - t5437 + t5441 + t5444 - t7751 - F::new(0.571528e-1) * t7753 + F::new(0.80040858019733333332e-2) * t7756 - F::new(0.675260332e-1) * t1651 * t2769 - F::new(0.1350520664e0) * t596 * t7761;
    t7764
}
