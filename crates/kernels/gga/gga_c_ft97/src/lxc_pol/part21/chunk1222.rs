//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1222/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1222<F: Float>(t100089: F, t101975: F, t102439: F, t103955: F, t116749: F, t117486: F, t1286: F, t1564: F, t22935: F, t25577: F, t25861: F, t28: F, t29582: F, t29741: F, t3052: F, t3289: F, t4495: F, t497: F, t5501: F, t5507: F, t5508: F, t5620: F, t6455: F, t925: F, t94046: F, t94049: F) -> (F,) {
    let t118402 = -t1286 * t28 * t5507 * t497 * t4495 / 3.0 + t102439 + 2.0 / 27.0 * t94046 + 2.0 / 27.0 * t94049 - t1286 * t28 * t116749 * t5508 / 3.0 - 2.0 * t117486 + t1286 * t28 * t6455 * t3289 / 3.0 - t5501 * t1564 * t101975 * t925 / 9.0 - 2.0 / 9.0 * t25577 * t1564 * t25861 * t3052 - t22935 * t29582 / 9.0 - t5501 * t1564 * t100089 * t925 / 9.0 - t103955 + t29741 * t5620 / 6.0;
    (t118402,)
}
