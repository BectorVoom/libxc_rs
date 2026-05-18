//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 968/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk968<F: Float>(t46362: F, t6717: F, t6914: F, t10215: F, t10421: F, t1445: F, t1457: F, t204: F, t2778: F, t3338: F, t41891: F, t41900: F, t41903: F, t41909: F, t44395: F, t447: F, t46080: F, t46327: F, t46331: F, t46336: F, t46339: F, t46342: F, t46343: F, t46345: F, t46352: F, t46354: F, t46356: F, t46361: F, t574: F, t587: F, t7980: F) -> F {
    let t46365 = F::new(0.37959496694381542179e3) * t6914 * t6717 * t46362;
    let t46367 = -F::new(0.92023022289409799224e1) * t574 * t1445 * t7980 * t3338 - F::new(0.92023022289409799224e1) * t574 * t1445 * t2778 * t10215 + t46327 - F::new(0.76685851907841499353e0) * t41891 + F::new(0.76685851907841499353e0) * t41900 + F::new(0.57514388930881124515e0) * t46331 + F::new(0.38342925953920749677e1) * t41903 + t46336 - t46339 + t46342 + t46343 + t46345 - F::new(0.71500979903700853338e0) * t10421 * t1457 * t44395 * t447 + t46352 + t46354 + t46356 - F::new(0.18404604457881959845e2) * t587 * t204 * t46080 - t46361 - t46365 + F::new(0.59584149919750711116e-1) * t41909;
    t46367
}
