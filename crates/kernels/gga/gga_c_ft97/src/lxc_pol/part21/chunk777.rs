//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 777/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk777<F: Float>(t1322: F, t1636: F, t89: F, t375: F, t5700: F, t376: F, t5623: F, t1286: F, t23037: F, t1882: F, t5657: F, t1328: F, t1637: F, t5724: F, t1314: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23075 = t89 * t1636 * t1322;
    let t23076 = 4.0 / 9.0 * t23075;
    let t23081 = t89 * t375 * t5700;
    let t23089 = t376 * t5623;
    let t23090 = t1286 * t23089;
    let t23114 = 2.0 / 27.0 * t23037;
    let t23124 = 4.0 / 27.0 * t23075;
    let t23148 = t1882 * t5657;
    let t23152 = 4.0 / 27.0 * t89 * t1637 * t1328;
    let t23176 = t1882 * t5724;
    let t23183 = 4.0 / 27.0 * t8232 * t1314;
    (t23076, t23081, t23089, t23090, t23114, t23124, t23148, t23152, t23176, t23183)
}
