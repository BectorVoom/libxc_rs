//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2657/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2657<F: Float>(t13937: F, t9962: F, t13991: F, t13999: F, t13786: F, t13760: F, t9765: F, t13756: F, t3989: F, t268: F, t5617: F, t46784: F) -> (F, F, F, F, F, F, F) {
    let t48892 = t9962 * t13937;
    let t48900 = t13999 * t13991;
    let t48902 = t9962 * t13786;
    let t48904 = t9765 * t13760;
    let t48905 = F::cast_from(0.16262400898971305032e-2_f64) * t48904;
    let t48906 = t3989 * t13756;
    let t48908 = t5617 * t268;
    let t48909 = t46784 * t48908;
    (t48892, t48900, t48902, t48905, t48906, t48908, t48909)
}
