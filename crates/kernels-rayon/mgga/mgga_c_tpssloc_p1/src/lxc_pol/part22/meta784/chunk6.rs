//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2697/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2697(t1388: f64, t6330: f64, t6463: f64, t1307: f64, t15899: f64, t20563: f64, t3918: f64, t3919: f64, t39529: f64, t39539: f64, t39549: f64, t5126: f64, t5160: f64, t5161: f64, t74476: f64, t74477: f64, t74478: f64, t74479: f64) -> f64 {
    let t75203 = t6330 * t1388;
    let t75210 = t6463 * t1388;
    let t75214 = t6463 * t1307;
    let t75218 = 6.0_f64 * t15899 * t5160 * t75210 + 18.0_f64 * t20563 * t3919 * t5126 - 9.0_f64 * t3918 * t5161 * t75214 - 18.0_f64 * t5126 * t5161 * t75203 - t39529 + t39539 + t39549 - t74476 - t74477 - t74478 - t74479;
    t75218
}
