//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1265/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1265<F: Float>(t1368: F, t16842: F, t16845: F, t21061: F, t21065: F, t21069: F, t21074: F, t21079: F, t21084: F, t21088: F, t21098: F, t5691: F, t5702: F, t5706: F, t5710: F) -> F {
    let t21101 = F::new(11.0) / F::new(324.0) * t21061 - t1368 * t21065 / F::new(288.0) - t1368 * t21069 / F::new(288.0) - t1368 * t21074 / F::new(144.0) + t1368 * t21079 / F::new(216.0) + t1368 * t21084 / F::new(144.0) - t21088 / F::new(432.0) + t5691 * t5706 / F::new(54.0) + t5691 * t5710 / F::new(27.0) - F::new(2.0) / F::new(81.0) * t5691 * t5702 + t16842 / F::new(216.0) + t16845 + t1368 * t21098 / F::new(72.0);
    t21101
}
