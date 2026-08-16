//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2605/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2605<F: Float>(t3575: F, t373: F, t470: F, t493: F, t1214: F, t820: F, t3624: F, t52627: F, t11745: F, t15503: F, t15737: F, t3493: F, t475: F, t607: F) -> (F, F, F, F, F, F) {
    let t52893 = t470 * t493 * t3575 * t373;
    let t52897 = t820 * t1214;
    let t52903 = t3624 * t52627;
    let t52906 = t15503 * t11745;
    let t52908 = t15737 * t11745;
    let t52911 = t607 * t3493 * t475;
    (t52893, t52897, t52903, t52906, t52908, t52911)
}
