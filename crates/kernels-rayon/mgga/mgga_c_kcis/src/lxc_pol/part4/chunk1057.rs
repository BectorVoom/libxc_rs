//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1057/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1057(t3191: f64, t4999: f64, t1092: f64, t1768: f64, t9539: f64, t3178: f64, t4773: f64, t2811: f64, t4977: f64, t1008: f64, t2822: f64, t5006: f64) -> (f64, f64, f64, f64, f64) {
    let t13366 = t4999 * t3191;
    let t13367 = t1092 * t13366;
    let t13369 = t9539 * t1768;
    let t13370 = t1092 * t13369;
    let t13372 = t3178 * t4773;
    let t13373 = t1092 * t13372;
    let t13376 = t4977 * t2811;
    let t13377 = t13376 * t1008;
    let t13382 = t2822 * t5006;
    (t13367, t13370, t13373, t13377, t13382)
}
