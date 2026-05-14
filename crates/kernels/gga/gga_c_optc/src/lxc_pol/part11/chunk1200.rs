//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1200/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1200<F: Float>(t5053: F, t2549: F, t2569: F, t277: F, t39288: F, t3980: F, t49803: F, t49808: F, t5059: F, t56954: F, t56957: F, t57022: F, t57113: F, t57117: F, t57120: F, t57185: F, t57213: F, t914: F, t95: F, t999: F) -> (F,) {
    let t58190 = t5053 * t5053;
    let t58195 = t56954 + t56957 + t57113 + t57117 + 0.31013857721884116596e-1 * t3980 * t39288 * t5059 + 8.0 * t999 * t914 * t2549 * t57022 + t57120 - 4.0 / 3.0 * t49803 + 4.0 / 3.0 * t49808 - t57185 + t57213 - 0.77534644304710291488e-2 * t95 * t277 * t58190 * t2569;
    (t58195,)
}
