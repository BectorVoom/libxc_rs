//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1059/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1059<F: Float>(t1844: F, t3643: F, t13238: F, t13242: F, t13219: F, t13222: F, t13225: F, t13228: F, t13231: F, t3617: F, t3623: F, t5345: F, t9522: F, t9524: F, t3616: F, t5281: F) -> (F, F, F) {
    let t15134 = t1844 * t3643;
    let t15157 = 0.61905925925925925925e-2 * t13238;
    let t15158 = 0.25794135802469135802e-2 * t13242;
    let t15159 = 0.77382407407407407407e-3 * t13219 - 0.41270617283950617284e-2 * t13222 + 0.12381185185185185185e-1 * t13225 - 0.10317654320987654321e-1 * t13228 + 0.92858888888888888886e-2 * t13231 + 0.77382407407407407407e-3 * t9522 - 0.23214722222222222222e-2 * t9524 - 0.66725e-1 * t5345 * t3617 + 0.66725e-1 * t5345 * t3623 - t15157 + t15158;
    let t15168 = t5281 * t3616;
    (t15134, t15159, t15168)
}
