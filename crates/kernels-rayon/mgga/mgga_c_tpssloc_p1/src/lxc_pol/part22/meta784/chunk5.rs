//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2696/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2696(t1390: f64, t16497: f64, t193: f64, t19577: f64, t19631: f64, t20063: f64, t20067: f64, t20077: f64, t20085: f64, t3918: f64, t39483: f64, t5122: f64, t5126: f64, t5160: f64, t5161: f64, t5308: f64, t533: f64, t54409: f64, t6330: f64, t74086: f64, t74470: f64, t74868: f64, t74899: f64, t74929: f64, t75183: f64) -> f64 {
    let t75198 = 18.0_f64 * t5126 * t20067 * t5308 + 9.0_f64 * t3918 * t5122 * t19631 + t54409 + t74086 + 18.0_f64 * t5126 * t16497 * t6330 + t39483 + t193 * t533 * (t74868 + t74899 + t74929 + t75183) * t1390 - 3.0_f64 * t5160 * t5161 * t20063 - 18.0_f64 * t5126 * t20077 * t5308 + 18.0_f64 * t3918 * t20085 * t19577 - t74470;
    t75198
}
