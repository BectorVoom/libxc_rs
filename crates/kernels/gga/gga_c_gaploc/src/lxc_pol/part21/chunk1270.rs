//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1270/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1270<F: Float>(t33557: F, t6066: F, t7630: F, t32356: F, t739: F, t1991: F, t590: F, t10938: F, t2021: F, t23310: F, t25177: F, t959: F) -> (F, F, F, F) {
    let t33560 = F::cast_from(0.14300195980740170668e1_f64) * t7630 * t6066 * t33557;
    let t33561 = t739 * t32356;
    let t33564 = F::cast_from(0.2044956050875773316e1_f64) * t1991 * t33561 * t590;
    let t33565 = t2021 * t10938;
    let t33567 = F::cast_from(0.79445533226334281486e-1_f64) * t33565 * t23310;
    let t33568 = t25177 * t959;
    (t33560, t33564, t33567, t33568)
}
