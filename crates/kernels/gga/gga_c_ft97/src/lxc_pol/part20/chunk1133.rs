//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1133/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1133<F: Float>(t6932: F, t8232: F, t28302: F, t8392: F, t28295: F, t10085: F, t1091: F, t11593: F, t13672: F, t13839: F, t1456: F, t1901: F, t24600: F, t2469: F, t2599: F, t28157: F, t28246: F, t446: F, t729: F, t97269: F, t97271: F, t97273: F, t97275: F, t97277: F, t97283: F, t97285: F, t97676: F) -> (F,) {
    let t109936 = t8232 * t6932;
    let t109960 = 4.0 / 3.0 * t8392 * t28302;
    let t109962 = 4.0 / 9.0 * t8392 * t28295;
    let t109963 = -8.0 / 27.0 * t97269 - 2.0 / 9.0 * t97271 + t97273 / 9.0 + t97275 / 9.0 + 8.0 / 27.0 * t109936 + 2.0 / 3.0 * t446 * t729 * t2469 * t28246 - 2.0 / 9.0 * t97277 - 2.0 / 9.0 * t97283 - 4.0 / 9.0 * t97285 - 4.0 / 9.0 * t11593 * t10085 * t28157 + t1901 * t2599 * t97676 * t1091 / 9.0 - t446 * t729 * t1456 * t13672 / 3.0 + 2.0 / 9.0 * t1901 * t13839 * t24600 + t109960 + t109962;
    (t109963,)
}
