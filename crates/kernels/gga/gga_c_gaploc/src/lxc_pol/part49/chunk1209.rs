//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1209/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1209<F: Float>(t13813: F, t1562: F, t4614: F, t12078: F, t1415: F, t7030: F, t47953: F, t6716: F, t6717: F, t42422: F, t42425: F, t42429: F, t42432: F, t42435: F, t42438: F, t42442: F, t42444: F, t42448: F) -> F {
    let t48205 = t1562 * t4614 * t13813;
    let t48208 = t1415 * t12078 * t7030;
    let t48211 = t6716 * t6717 * t47953;
    let t48215 = -F::new(0.19171462976960374838e0) * t42422 - F::new(0.19171462976960374838e0) * t42425 + t42429 - t42432 - F::new(0.92023022289409799224e1) * t48205 - F::new(0.14896037479937677779e-1) * t48208 + F::new(0.69017266717057349418e1) * t48211 - F::new(0.79445533226334281487e-1) * t42435 + t42438 + t42442 - t42444 - F::new(0.7150097990370085334e0) * t42448;
    t48215
}
