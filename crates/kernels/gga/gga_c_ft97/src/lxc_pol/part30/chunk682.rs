//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 682/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk682<F: Float>(t33460: F, t505: F, t9770: F, t446: F, t33243: F, t713: F, t193: F, t89: F, t6008: F, t6061: F, t375: F, t7532: F, t668: F, t7484: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33462 = t9770 * t33460 * t505;
    let t33463 = t446 * t33462;
    let t33465 = t33243 * t713;
    let t33466 = t193 * t33465;
    let t33467 = t89 * t33466;
    let t33469 = t6008 * t6061;
    let t33470 = t193 * t33469;
    let t33471 = t89 * t33470;
    let t33474 = t89 * t375 * t7532;
    let t33475 = t33474 / 3.0;
    let t33476 = t7484 * t668;
    (t33462, t33463, t33465, t33467, t33469, t33471, t33474, t33475, t33476)
}
