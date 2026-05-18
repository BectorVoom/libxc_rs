//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 927/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk927<F: Float>(t3375: F, t3665: F, t3775: F, t3806: F, t1029: F, t3237: F, t1020: F, t3228: F, t879: F, t1036: F, t174: F, t386: F, t387: F) -> (F, F, F, F, F, F) {
    let t14239 = t3375 * t3665;
    let t14242 = F::new(0.51448821741683684368e-2) * t3775 * t3806;
    let t14243 = t3237 * t1029;
    let t14245 = t3228 * t1020;
    let t14255 = t879 * t879;
    let t14260 = F::new(0.12862205435420921092e-2) * t1036 * t386 * t387 * t174 * t14255;
    (t14239, t14242, t14243, t14245, t14255, t14260)
}
