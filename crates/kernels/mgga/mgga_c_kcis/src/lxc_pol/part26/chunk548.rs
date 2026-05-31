//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 548/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk548<F: Float>(t1378: F, t5732: F, t286: F, t1368: F, t1373: F, t1382: F, t1930: F, t3969: F, t3972: F, t3975: F, t493: F, t5689: F, t5691: F, t5699: F, t5702: F, t5706: F, t5710: F, t5715: F, t5719: F, t5723: F, t5728: F) -> (F, F, F) {
    let t5733 = t1378 * t5732;
    let t5734 = t286 * t5733;
    let t5737 = -t5689 / F::cast_from(108.0_f64) - t5691 * t1373 / F::cast_from(108.0_f64) + t1930 * t1382 / F::cast_from(36.0_f64) - t3969 + t3972 / F::cast_from(864.0_f64) - t3975 / F::cast_from(288.0_f64) + t5699 / F::cast_from(864.0_f64) + t1368 * t5702 / F::cast_from(216.0_f64) - t1368 * t5706 / F::cast_from(288.0_f64) - t1368 * t5710 / F::cast_from(144.0_f64) - t1368 * t5715 / F::cast_from(144.0_f64) - t5719 / F::cast_from(288.0_f64) - t1368 * t5723 / F::cast_from(288.0_f64) + t1368 * t5728 / F::cast_from(48.0_f64) - t493 * t5734 / F::cast_from(96.0_f64);
    (t5733, t5734, t5737)
}
