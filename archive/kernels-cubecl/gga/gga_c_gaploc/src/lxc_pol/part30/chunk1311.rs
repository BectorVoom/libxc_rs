//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1311/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1311<F: Float>(t3484: F, t6021: F, t10973: F, t2194: F, t32435: F, t701: F, t6066: F, t7630: F, t32356: F, t739: F, t1991: F, t590: F) -> (F, F, F, F, F) {
    let t33544 = F::cast_from(0.46011511144704899612e1_f64) * t6021 * t3484;
    let t33546 = F::cast_from(0.92023022289409799224e1_f64) * t2194 * t10973;
    let t33557 = t32435 * t701;
    let t33560 = F::cast_from(0.14300195980740170668e1_f64) * t7630 * t6066 * t33557;
    let t33561 = t739 * t32356;
    let t33564 = F::cast_from(0.2044956050875773316e1_f64) * t1991 * t33561 * t590;
    (t33544, t33546, t33557, t33560, t33564)
}
