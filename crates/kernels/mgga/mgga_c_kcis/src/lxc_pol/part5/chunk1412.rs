//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1412/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1412<F: Float>(t22411: F, t22413: F, t22415: F, t22417: F, t22420: F, t22423: F, t22425: F, t22428: F, t22431: F, t22433: F, t22437: F, t22471: F, t22632: F, t22634: F, t22638: F, t22641: F, t22643: F, t22645: F, t22647: F, t22650: F, t22653: F, t22655: F) -> (F, F) {
    let t23321 = -F::cast_from(0.20833333333333333333e-1_f64) * t22411 + F::cast_from(0.26979166666666666666e-1_f64) * t22413 - F::cast_from(0.20234375e-1_f64) * t22415 - F::cast_from(0.125e0_f64) * t22417 + F::cast_from(0.61111111111111111111e0_f64) * t22420 - F::cast_from(0.45564814814814814815e0_f64) * t22423 + F::cast_from(0.20234375e-1_f64) * t22425 + F::cast_from(0.34173611111111111111e0_f64) * t22428 - F::cast_from(0.4046875e-1_f64) * t22431 - F::cast_from(0.20833333333333333333e-1_f64) * t22433 + F::cast_from(0.625e-1_f64) * t22437;
    let t23346 = -F::cast_from(0.44965277777777777777e-2_f64) * t22471 + F::cast_from(0.9375e-1_f64) * t22632 + F::cast_from(0.1875e0_f64) * t22634 - F::cast_from(0.101171875e-1_f64) * t22638 + F::cast_from(0.13489583333333333333e-1_f64) * t22641 + F::cast_from(0.10791666666666666667e0_f64) * t22643 - F::cast_from(0.1875e0_f64) * t22645 + F::cast_from(0.5e0_f64) * t22647 + F::cast_from(0.60703125e-1_f64) * t22650 - F::cast_from(0.10791666666666666667e0_f64) * t22653 + F::cast_from(0.101171875e-1_f64) * t22655;
    (t23321, t23346)
}
