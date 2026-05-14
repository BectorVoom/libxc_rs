//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1220/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1220<F: Float>(t34488: F, t6895: F, t9263: F, t993: F, t10604: F, t1415: F, t1646: F, t30705: F, t30708: F, t34454: F, t34458: F, t34462: F, t34465: F, t34467: F, t34470: F, t34473: F, t34477: F, t34478: F, t34484: F, t34486: F, t4425: F) -> (F,) {
    let t34489 = 0.76685851907841499352e0 * t34488;
    let t34491 = t9263 * t993 * t6895;
    let t34492 = 0.38342925953920749676e0 * t34491;
    let t34493 = -t34454 - 0.51123901271894332905e0 * t4425 * t10604 + t34458 - t34462 + t34465 + t34467 + t34470 - t34473 + t34477 - 0.71500979903700853338e0 * t1415 * t34478 * t1646 + t34484 + t30705 - t30708 - t34486 - t34489 - t34492;
    (t34493,)
}
