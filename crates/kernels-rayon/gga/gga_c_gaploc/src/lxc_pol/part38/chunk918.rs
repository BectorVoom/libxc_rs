//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 918/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk918(t11627: f64, t1445: f64, t2530: f64, t833: f64, t13598: f64, t5771: f64, t1457: f64, t2103: f64, t44973: f64, t45087: f64, t13602: f64, t2194: f64) -> (f64, f64, f64, f64, f64) {
    let t45598 = 0.43710935587469654631e2_f64 * t833 * t1445 * t11627 * t2530;
    let t45600 = 0.71500979903700853338e0_f64 * t5771 * t13598;
    let t45603 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t44973;
    let t45606 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t45087;
    let t45608 = 0.92023022289409799224e1_f64 * t2194 * t13602;
    (t45598, t45600, t45603, t45606, t45608)
}
