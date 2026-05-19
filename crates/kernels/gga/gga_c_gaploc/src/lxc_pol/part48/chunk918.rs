//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 918/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk918<F: Float>(t11627: F, t1445: F, t2530: F, t833: F, t13598: F, t5771: F, t1457: F, t2103: F, t44973: F, t45087: F, t13602: F, t2194: F) -> (F, F, F, F, F) {
    let t45598 = F::cast_from(0.43710935587469654631e2_f64) * t833 * t1445 * t11627 * t2530;
    let t45600 = F::cast_from(0.71500979903700853338e0_f64) * t5771 * t13598;
    let t45603 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t44973;
    let t45606 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t45087;
    let t45608 = F::cast_from(0.92023022289409799224e1_f64) * t2194 * t13602;
    (t45598, t45600, t45603, t45606, t45608)
}
