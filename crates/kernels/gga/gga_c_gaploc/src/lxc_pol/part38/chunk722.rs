//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 722/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk722<F: Float>(t10677: F, t2464: F, t2465: F, t825: F, t10782: F, t2684: F, t13072: F, t32757: F, t25359: F, t2615: F, t9438: F, t41448: F, t41477: F, t2344: F, t550: F, t1358: F, t161: F, t37975: F) -> (F, F, F, F, F, F, F, F) {
    let t44124 = t825 * t2464 * t2465 * t10677;
    let t44128 = t2684 * t2464 * t2465 * t10782;
    let t44130 = t32757 * t13072;
    let t44133 = t2615 * t9438 * t25359;
    let t44147 = 0.31952438294933958063e0 * t41448;
    let t44157 = 0.12780975317973583225e0 * t41477;
    let t44255 = t550 * t2344;
    let t44258 = 0.37940008847568199464e-1 * t1358 * t37975 * t161 * t44255;
    (t44124, t44128, t44130, t44133, t44147, t44157, t44255, t44258)
}
