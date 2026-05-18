//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 595/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk595<F: Float>(t1378: F, t5732: F, t286: F, t1368: F, t1373: F, t1382: F, t1930: F, t3969: F, t3972: F, t3975: F, t493: F, t5689: F, t5691: F, t5699: F, t5702: F, t5706: F, t5710: F, t5715: F, t5719: F, t5723: F, t5728: F) -> (F, F) {
    let t5733 = t1378 * t5732;
    let t5734 = t286 * t5733;
    let t5737 = -t5689 / F::new(108.0) - t5691 * t1373 / F::new(108.0) + t1930 * t1382 / F::new(36.0) - t3969 + t3972 / F::new(864.0) - t3975 / F::new(288.0) + t5699 / F::new(864.0) + t1368 * t5702 / F::new(216.0) - t1368 * t5706 / F::new(288.0) - t1368 * t5710 / F::new(144.0) - t1368 * t5715 / F::new(144.0) - t5719 / F::new(288.0) - t1368 * t5723 / F::new(288.0) + t1368 * t5728 / F::new(48.0) - t493 * t5734 / F::new(96.0);
    (t5733, t5737)
}
