//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1269/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1269<F: Float>(t28640: F, t33533: F, t7419: F, t3005: F, t7383: F, t9800: F, t3484: F, t6021: F, t10973: F, t2194: F, t32435: F, t701: F) -> (F, F, F, F, F) {
    let t33535 = t28640 * t33533 * t7419;
    let t33536 = F::cast_from(0.23005755572352449806e1_f64) * t33535;
    let t33538 = t9800 * t3005 * t7383;
    let t33539 = F::cast_from(0.9585731488480187419e0_f64) * t33538;
    let t33544 = F::cast_from(0.46011511144704899612e1_f64) * t6021 * t3484;
    let t33546 = F::cast_from(0.92023022289409799224e1_f64) * t2194 * t10973;
    let t33557 = t32435 * t701;
    (t33536, t33539, t33544, t33546, t33557)
}
