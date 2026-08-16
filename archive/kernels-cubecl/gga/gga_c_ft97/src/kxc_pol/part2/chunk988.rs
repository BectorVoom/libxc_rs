//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 988/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk988<F: Float>(t4176: F, t684: F, t10703: F, t2843: F, t848: F, t4181: F, t1221: F, t8232: F, t15134: F, t296: F, t15140: F, t1242: F, t2399: F, t89: F) -> (F, F, F, F, F, F) {
    let t15308 = t4176 * t684;
    let t15309 = t10703 * t15308;
    let t15312 = t848 * t2843;
    let t15313 = t4181 * t684;
    let t15314 = t15312 * t15313;
    let t15318 = t8232 * t1221;
    let t15322 = t296 * t15134;
    let t15325 = t296 * t15140;
    let t15329 = t89 * t2399 * t1242;
    (t15309, t15314, t15318, t15322, t15325, t15329)
}
