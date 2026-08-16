//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1507/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1507(t17422: f64, t913: f64, t893: f64, t10655: f64, t5730: f64, t5737: f64, t923: f64, t5775: f64, t950: f64, t1581: f64, t4471: f64, t10740: f64, t14263: f64, t14266: f64, t14337: f64, t1569: f64, t17377: f64, t17379: f64, t2856: f64, t2905: f64, t2930: f64, t4411: f64, t4434: f64, t4454: f64, t4476: f64, t5743: f64, t5759: f64, t933: f64) -> (f64, f64, f64) {
    let t17423 = t17422 * t913;
    let t17425 = 1.0_f64 * t893 * t17423;
    let t17427 = 0.16081979498692535067e2_f64 * t10655 * t5730;
    let t17428 = t5737 * t923;
    let t17443 = t5775 * t950;
    let t17446 = t1581 * t4471;
    let t17449 = t17377 - t17379 - t17425 - t17427 + 1.0_f64 * t17428 * t933 + 2.0_f64 * t14266 * t1569 + 2.0_f64 * t4411 * t4434 - 2.0_f64 * t10740 * t5743 + 1.0_f64 * t2856 * t5759 - 0.23392894490538584828e1_f64 * t14263 * t4454 + 0.34631718211362927517e2_f64 * t14337 * t4476 + 0.35089341735807877242e1_f64 * t2930 * t17443 - 0.23392894490538584828e1_f64 * t2905 * t17446;
    (t17425, t17427, t17449)
}
