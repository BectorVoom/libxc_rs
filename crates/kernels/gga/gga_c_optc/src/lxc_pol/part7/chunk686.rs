//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 686/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk686<F: Float>(t1784: F, t1790: F, t1792: F, t533: F, t587: F, t6407: F, t1859: F, t588: F, t534: F, t6434: F, t1785: F, t1835: F, t209: F) -> (F, F, F, F, F, F) {
    let t6465 = F::cast_from(0.48245472966453314466e2_f64) * t1790 * t1784 * t1792 * t533;
    let t6466 = t6407 * t587;
    let t6472 = t588 * t1859;
    let t6475 = t6434 * t534;
    let t6477 = F::new(6.0) * t1790 * t6475;
    let t6480 = F::new(0.53425e-1) * t209 * t1835 * t1785;
    (t6465, t6466, t6472, t6475, t6477, t6480)
}
