//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 826/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk826<F: Float>(t10782: F, t2464: F, t2465: F, t2684: F, t13072: F, t32757: F, t25359: F, t2615: F, t9438: F, t41448: F, t41477: F, t2344: F, t550: F) -> (F, F, F, F, F, F) {
    let t44128 = t2684 * t2464 * t2465 * t10782;
    let t44130 = t32757 * t13072;
    let t44133 = t2615 * t9438 * t25359;
    let t44147 = F::cast_from(0.31952438294933958063e0_f64) * t41448;
    let t44157 = F::cast_from(0.12780975317973583225e0_f64) * t41477;
    let t44255 = t550 * t2344;
    (t44128, t44130, t44133, t44147, t44157, t44255)
}
