//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 714/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk714<F: Float>(t14302: F, t1445: F, t14271: F, t12054: F, t12881: F, t13354: F, t13356: F, t13365: F, t13370: F, t13374: F, t13775: F, t13776: F, t14299: F, t1562: F, t2877: F, t3702: F, t574: F, t597: F) -> (F, F, F) {
    let t14303 = t1445 * t14302;
    let t14306 = t1445 * t14271;
    let t14313 = t13354 + t13356 + t13365 - t13775 + t13776 - t13370 - t13374 - F::new(0.13803453343411469884e2) * t1562 * t14299 - F::new(0.92023022289409799224e1) * t574 * t14303 + F::new(0.23005755572352449806e2) * t597 * t14306 + F::new(0.71500979903700853338e0) * t3702 * t2877 - F::new(0.21450293971110256002e1) * t12054 * t12881;
    (t14303, t14306, t14313)
}
