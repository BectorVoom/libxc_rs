//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 936/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk936<F: Float>(t1198: F, t1426: F, t2297: F, t598: F, t30374: F, t8477: F, t1181: F, t4555: F, t599: F, t7493: F, t4701: F, t7561: F, t4447: F, t4384: F, t8511: F, t30371: F, t4376: F) -> (F, F, F, F, F, F, F) {
    let t34446 = t598 * t1426 * t1198 * t2297;
    let t34449 = t30374 * t8477;
    let t34453 = t7493 * t1181 * t599 * t4555;
    let t34455 = t7561 * t4701;
    let t34457 = t7561 * t4447;
    let t34459 = t8511 * t4384;
    let t34461 = t30371 * t4376;
    (t34446, t34449, t34453, t34455, t34457, t34459, t34461)
}
