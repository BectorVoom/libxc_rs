//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 997/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk997<F: Float>(t40013: F, t40015: F, t40019: F, t12000: F, t123: F, t883: F, t2487: F, t2488: F, t11981: F, t2464: F, t2465: F, t13782: F, t7014: F, t41718: F, t41719: F, t41721: F, t41724: F, t41729: F, t41731: F) -> (F, F) {
    let t47873 = 0.63904876589867916128e-1 * t40013;
    let t47874 = 0.63904876589867916128e-1 * t40015;
    let t47875 = 0.63904876589867916128e-1 * t40019;
    let t47877 = t12000 * t123 * t883;
    let t47879 = t2487 * t2488 * t47877;
    let t47883 = t2487 * t2464 * t2465 * t11981;
    let t47885 = t7014 * t13782;
    let t47889 = -t47873 + t47874 + t47875 + t41718 + t41719 + 0.19171462976960374838e0 * t47879 - 0.42603251059911944084e-1 * t47883 + 0.19171462976960374838e0 * t47885 + t41721 - 0.35750489951850426669e0 * t41724 - t41729 + 0.29792074959875355558e-1 * t41731;
    (t47877, t47889)
}
