//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 947/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk947<F: Float>(t14381: F, t14386: F, t4554: F, t1714: F, t9562: F, t20: F, t284: F, t2194: F, t2909: F, t992: F, t1704: F, t2895: F) -> (F, F, F, F, F) {
    let t14387 = t14381 * t14386;
    let t14388 = t4554 * t14387;
    let t14390 = t9562 * t1714;
    let t14393 = t284 * t20;
    let t14394 = t14393 * t2194;
    let t14395 = t992 * t2909;
    let t14396 = t14395 * t1704;
    let t14397 = t14396 * t2895;
    (t14388, t14390, t14394, t14395, t14397)
}
