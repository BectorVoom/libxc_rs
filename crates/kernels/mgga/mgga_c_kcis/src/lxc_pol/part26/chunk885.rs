//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 885/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk885<F: Float>(t1368: F, t16842: F, t16845: F, t21061: F, t21065: F, t21069: F, t21074: F, t21079: F, t21084: F, t21088: F, t21098: F, t5691: F, t5702: F, t5706: F, t5710: F) -> F {
    let t21101 = F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t21061 - t1368 * t21065 / F::cast_from(288.0_f64) - t1368 * t21069 / F::cast_from(288.0_f64) - t1368 * t21074 / F::cast_from(144.0_f64) + t1368 * t21079 / F::cast_from(216.0_f64) + t1368 * t21084 / F::cast_from(144.0_f64) - t21088 / F::cast_from(432.0_f64) + t5691 * t5706 / F::cast_from(54.0_f64) + t5691 * t5710 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t5691 * t5702 + t16842 / F::cast_from(216.0_f64) + t16845 + t1368 * t21098 / F::cast_from(72.0_f64);
    t21101
}
