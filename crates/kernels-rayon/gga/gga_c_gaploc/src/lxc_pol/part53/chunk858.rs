//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 858/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk858(t1457: f64, t1572: f64, t41778: f64, t12922: f64, t26939: f64, t12926: f64, t1641: f64, t1445: f64, t3116: f64, t574: f64, t7980: f64, t2778: f64, t9127: f64) -> (f64, f64, f64, f64, f64) {
    let t42278 = 0.71500979903700853338e0_f64 * t1572 * t1457 * t41778;
    let t42282 = 0.42900587942220512003e1_f64 * t26939 * t12922;
    let t42284 = 0.46011511144704899612e1_f64 * t1641 * t12926;
    let t42288 = 0.46011511144704899612e1_f64 * t574 * t1445 * t7980 * t3116;
    let t42292 = 0.46011511144704899612e1_f64 * t574 * t1445 * t2778 * t9127;
    (t42278, t42282, t42284, t42288, t42292)
}
