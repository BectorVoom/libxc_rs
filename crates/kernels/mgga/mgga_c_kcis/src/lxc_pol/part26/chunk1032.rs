//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1032/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1032<F: Float>(t22411: F, t22413: F, t22415: F, t22417: F, t22420: F, t22423: F, t22425: F, t22428: F, t22431: F, t22433: F, t22437: F, t22471: F, t22632: F, t22634: F, t22638: F, t22641: F, t22643: F, t22645: F, t22647: F, t22650: F, t22653: F, t22655: F) -> (F, F) {
    let t23321 = -F::new(0.20833333333333333333e-1) * t22411 + F::new(0.26979166666666666666e-1) * t22413 - F::new(0.20234375e-1) * t22415 - F::new(0.125e0) * t22417 + F::new(0.61111111111111111111e0) * t22420 - F::new(0.45564814814814814815e0) * t22423 + F::new(0.20234375e-1) * t22425 + F::new(0.34173611111111111111e0) * t22428 - F::new(0.4046875e-1) * t22431 - F::new(0.20833333333333333333e-1) * t22433 + F::new(0.625e-1) * t22437;
    let t23346 = -F::new(0.44965277777777777777e-2) * t22471 + F::new(0.9375e-1) * t22632 + F::new(0.1875e0) * t22634 - F::new(0.101171875e-1) * t22638 + F::new(0.13489583333333333333e-1) * t22641 + F::new(0.10791666666666666667e0) * t22643 - F::new(0.1875e0) * t22645 + F::new(0.5e0) * t22647 + F::new(0.60703125e-1) * t22650 - F::new(0.10791666666666666667e0) * t22653 + F::new(0.101171875e-1) * t22655;
    (t23321, t23346)
}
