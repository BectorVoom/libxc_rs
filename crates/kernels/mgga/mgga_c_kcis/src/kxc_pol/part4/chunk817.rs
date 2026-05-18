//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 817/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk817<F: Float>(t5081: F, t5188: F, t1142: F, t2919: F, t3537: F, t4612: F, t4615: F, t4618: F, t4623: F, t1211: F, t1823: F, t1219: F, t1831: F) -> (F, F, F, F, F) {
    let t5189 = t5081 + t5188;
    let t5190 = t1142 * t5189;
    let t5208 = t3537 + F::new(0.57077777777777777777e-2) * t2919 + F::new(0.57077777777777777777e-2) * t4612 - F::new(0.11415555555555555555e-1) * t4615 + F::new(0.34246666666666666666e-1) * t4618 - F::new(0.34246666666666666666e-1) * t4623;
    let t5211 = t1823 * t1211;
    let t5216 = t1831 * t1219;
    (t5189, t5190, t5208, t5211, t5216)
}
