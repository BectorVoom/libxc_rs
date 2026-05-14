//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1118/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1118<F: Float>(t1286: F, t29464: F, t376: F, t29605: F, t8466: F, t100127: F, t100128: F, t102037: F, t102049: F, t102051: F, t102291: F, t102385: F, t102442: F, t1564: F, t15951: F, t15955: F, t15959: F, t16120: F, t22935: F, t25584: F, t28: F, t29594: F, t3289: F, t5501: F, t5502: F, t5507: F, t6461: F, t8411: F, t925: F, t942: F) -> (F, F) {
    let t115277 = t1286 * t376 * t29464;
    let t115289 = t8466 * t29605;
    let t115306 = t25584 * t6461 / 3.0 - t115277 / 9.0 + 4.0 / 27.0 * t102037 - t5501 * t1564 * t102291 * t925 / 9.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t3289 * t942 + 8.0 * t115289 - 2.0 / 3.0 * t100127 * t100128 * t15959 - 4.0 / 9.0 * t100127 * t102385 * t15951 + 4.0 / 27.0 * t100127 * t102442 * t15955 - t102049 - t102051 + 2.0 * t5501 * t8411 * t5502 * t16120 + t22935 * t29594 / 9.0;
    (t115289, t115306)
}
