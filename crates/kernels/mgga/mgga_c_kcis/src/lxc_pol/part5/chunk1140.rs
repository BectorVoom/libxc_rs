//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1140/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1140<F: Float>(t1001: F, t19180: F, t286: F, t14394: F, t14423: F, t14427: F, t14439: F, t14442: F, t14446: F, t14450: F, t14455: F, t19166: F, t19173: F, t19176: F, t285: F, t9614: F) -> F {
    let t19181 = t1001 * t19180;
    let t19182 = t286 * t19181;
    let t19186 = -t14394 * t19166 / F::new(108.0) + t9614 / F::new(432.0) + t14423 / F::new(216.0) - t14427 + t14439 + t14394 * t19173 / F::new(72.0) + t14394 * t19176 / F::new(72.0) - t285 * t19182 / F::new(96.0) - t14442 - t14446 + t14450 + t14455 / F::new(216.0);
    t19186
}
