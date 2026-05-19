//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 529/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk529<F: Float>(t9561: F, t9562: F, t549: F, t9199: F, t1407: F, t3178: F, t3163: F, t4379: F, t2293: F, t2366: F, t2365: F, t1429: F) -> (F, F, F, F, F, F) {
    let t9564 = F::cast_from(0.89376224879626066674e-1_f64) * t9561 * t9562;
    let t9565 = t549 * t9199;
    let t9568 = t1407 * t3178;
    let t9569 = F::cast_from(0.38342925953920749676e0_f64) * t9568;
    let t9571 = F::cast_from(0.29792074959875355558e-1_f64) * t4379 * t3163;
    let t9572 = t2366 * t2293;
    let t9573 = t2365 * t9572;
    let t9575 = F::cast_from(0.29792074959875355558e-1_f64) * t1429 * t9573;
    (t9564, t9565, t9568, t9569, t9571, t9575)
}
