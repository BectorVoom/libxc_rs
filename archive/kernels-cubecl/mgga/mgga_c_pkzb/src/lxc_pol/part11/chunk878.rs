//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 878/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk878<F: Float>(t722: F, t9203: F, t5522: F, t5852: F, t7336: F, t7357: F, t9138: F, t9140: F, t9143: F, t9148: F, t9163: F, t9165: F, t9172: F, t9174: F) -> (F, F) {
    let t9465 = t9203 * t722;
    let t9482 = F::cast_from(0.264729375e1_f64) * t9138 - F::cast_from(0.3529725e1_f64) * t9140 - F::cast_from(0.17648625e1_f64) * t9143 + F::cast_from(0.3529725e1_f64) * t9165 - t5852 + F::cast_from(0.68863333333333333333e0_f64) * t5522 + F::cast_from(0.13772666666666666667e1_f64) * t7357 - t7336 - F::cast_from(0.516475e0_f64) * t9148 + F::cast_from(0.1549425e1_f64) * t9163 - F::cast_from(0.157790625e0_f64) * t9172 + F::cast_from(0.6311625e0_f64) * t9174;
    (t9465, t9482)
}
