//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1369/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1369<F: Float>(t21257: F, t21262: F, t21264: F, t21268: F, t21270: F, t21272: F, t21276: F, t26442: F, t26444: F, t26446: F, t26450: F, t28553: F, t28555: F, t21279: F, t21283: F, t21287: F, t21292: F, t21295: F, t21298: F, t21299: F, t21301: F, t21309: F, t21313: F, t21315: F, t26452: F, t26463: F, t28564: F) -> (F, F) {
    let t33466 = 12.0 * t28553 - t21257 - t21262 - t21264 - t21268 + t21270 + t21272 + t21276 + 0.17544670867903938621e1 * t28555 - 0.1301229756036208781e0 * t26442 + 0.19263893255070628431e1 * t26444 - 0.18676200204604444444e-1 * t26446 + t26450;
    let t33474 = -t26452 - t21279 - 3.0 * t28564 + 0.65061487801810439052e-1 * t21283 - 0.48159733137676571079e0 * t21287 + t21292 - t21295 - t21298 - 0.96319466275353142157e0 * t21299 - 0.97592231702715658578e-1 * t21301 + t21309 - t21313 - 0.3903689268108626343e0 * t21315 - 180.0 * t26463;
    (t33466, t33474)
}
