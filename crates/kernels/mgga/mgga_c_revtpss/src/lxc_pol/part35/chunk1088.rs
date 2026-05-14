//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1088/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1088<F: Float>(t114107: F, t114110: F, t114113: F, t114117: F, t114128: F, t114140: F, t114150: F, t114188: F, t115748: F, t1940: F, t2071: F, t22783: F, t2403: F, t26425: F, t28291: F, t28460: F, t29939: F, t29949: F, t29967: F, t29970: F, t30420: F, t33: F, t4541: F, t7432: F, t7862: F, t8020: F, t95964: F) -> (F,) {
    let t115913 = 9.0 * t28291 * t114128 - 3.0 * t1940 * t28460 * t29967 - 3.0 / 2.0 * t1940 * t28460 * t29970 - t1940 * t7432 * t114150 / 2.0 - 3.0 * t1940 * t95964 * t114188 + 9.0 * t4541 * t8020 * t29939 + 9.0 / 2.0 * t2403 * t30420 * t7862 - 9.0 / 2.0 * t26425 * t114110 + 9.0 / 2.0 * t2403 * t2071 * t114113 - 9.0 / 2.0 * t26425 * t114107 + t1940 * t2071 * t22783 / 2.0 + 9.0 * t4541 * t2071 * t114117 - 3.0 / 2.0 * t1940 * t7432 * t114140 + t1940 * t115748 * t33 / 2.0 + 9.0 * t2403 * t8020 * t29949;
    (t115913,)
}
