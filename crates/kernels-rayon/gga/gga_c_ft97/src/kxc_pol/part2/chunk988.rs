//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 988/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk988(t4176: f64, t684: f64, t10703: f64, t2843: f64, t848: f64, t4181: f64, t1221: f64, t8232: f64, t15134: f64, t296: f64, t15140: f64, t1242: f64, t2399: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
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
