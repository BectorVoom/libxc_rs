//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 968/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk968(t46362: f64, t6717: f64, t6914: f64, t10215: f64, t10421: f64, t1445: f64, t1457: f64, t204: f64, t2778: f64, t3338: f64, t41891: f64, t41900: f64, t41903: f64, t41909: f64, t44395: f64, t447: f64, t46080: f64, t46327: f64, t46331: f64, t46336: f64, t46339: f64, t46342: f64, t46343: f64, t46345: f64, t46352: f64, t46354: f64, t46356: f64, t46361: f64, t574: f64, t587: f64, t7980: f64) -> f64 {
    let t46365 = 0.37959496694381542179e3_f64 * t6914 * t6717 * t46362;
    let t46367 = -0.92023022289409799224e1_f64 * t574 * t1445 * t7980 * t3338 - 0.92023022289409799224e1_f64 * t574 * t1445 * t2778 * t10215 + t46327 - 0.76685851907841499353e0_f64 * t41891 + 0.76685851907841499353e0_f64 * t41900 + 0.57514388930881124515e0_f64 * t46331 + 0.38342925953920749677e1_f64 * t41903 + t46336 - t46339 + t46342 + t46343 + t46345 - 0.71500979903700853338e0_f64 * t10421 * t1457 * t44395 * t447 + t46352 + t46354 + t46356 - 0.18404604457881959845e2_f64 * t587 * t204 * t46080 - t46361 - t46365 + 0.59584149919750711116e-1_f64 * t41909;
    t46367
}
