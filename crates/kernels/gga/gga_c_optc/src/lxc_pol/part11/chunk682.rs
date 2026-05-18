//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 682/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk682<F: Float>(t6461: F, t6523: F, t60: F, t40: F, t47: F, t768: F, t1026: F, t52: F, t542: F, t8: F) -> (F, F, F, F, F, F) {
    let t6524 = t6461 + t6523;
    let t6525 = t60 * t6524;
    let t6526 = t40 * t6525;
    let t6533 = F::new(1.0) / t47 / t768;
    let t6547 = F::new(1.0) / t52 / t1026;
    let t6567 = F::new(1.0) / t8 / t542;
    (t6524, t6525, t6526, t6533, t6547, t6567)
}
