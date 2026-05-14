//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 732/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk732<F: Float>(t1017: F, t2180: F, t2179: F, t574: F, t1986: F, t167: F, t9432: F, t12001: F, t3471: F, t1060: F, t1647: F, t569: F, t13040: F, t13042: F, t13045: F, t13049: F, t13051: F, t13055: F, t13058: F, t13062: F, t1901: F, t446: F, t9321: F, t9340: F, t9342: F) -> (F,) {
    let t13065 = t1017 * t2180;
    let t13067 = t574 * t2179 * t13065;
    let t13070 = t1017 * t1986;
    let t13072 = t9432 * t167 * t13070;
    let t13075 = t12001 * t3471;
    let t13078 = t569 * t1060 * t1647;
    let t13081 = 8.0 / 27.0 * t9321 - t13040 - t13042 - 2.0 / 9.0 * t1901 * t13045 - t13049 - 2.0 / 9.0 * t1901 * t13051 - 2.0 / 3.0 * t1901 * t13055 + 4.0 / 3.0 * t446 * t13058 + t13062 + 2.0 / 9.0 * t9340 + 2.0 / 9.0 * t9342 - 2.0 / 3.0 * t446 * t13067 - 2.0 * t446 * t13072 + 22.0 / 27.0 * t13075 + 2.0 / 9.0 * t446 * t13078;
    (t13081,)
}
