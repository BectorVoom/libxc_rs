//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 475/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk475(t381: f64, t79: f64, t3784: f64, t492: f64, t306: f64, t476: f64, t140: f64, t430: f64, t480: f64, t11: f64, t139: f64) -> (f64, f64, f64, f64, f64) {
    let t4231 = t79 * t381;
    let t4235 = t3784 * t492;
    let t4253 = t476 * t306;
    let t4264 = 0.88437037037037037037e-2_f64 * t140 * t430 * t480;
    let t4265 = t139 * t11;
    (t4231, t4235, t4253, t4264, t4265)
}
