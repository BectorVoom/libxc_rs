//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 866/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk866<F: Float>(t6966: F, t7034: F, t2576: F, t993: F, t2537: F, t974: F, t2519: F, t947: F, t347: F) -> (F, F, F, F, F, F) {
    let t7116 = F::cast_from(0.93932222222222222223e0_f64) * t6966;
    let t7123 = F::cast_from(0.36793333333333333333e0_f64) * t7034;
    let t7133 = t993 * t2576;
    let t7140 = t974 * t2537;
    let t7147 = F::cast_from(1.0_f64) / t2519 / t947;
    let t7148 = t347 * t7147;
    (t7116, t7123, t7133, t7140, t7147, t7148)
}
