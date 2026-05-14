//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1071/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1071<F: Float>(t101785: F, t101929: F, t109926: F, t114260: F, t114270: F, t114288: F, t114296: F, t114301: F, t2048: F, t26175: F, t28602: F, t29538: F, t29544: F, t29548: F, t29562: F, t30543: F, t7343: F, t7706: F, t7709: F, t7964: F) -> (F,) {
    let t115348 = -2.0 * t7709 * t30543 + 88.0 / 9.0 * t101929 + 30.0 * t101785 * t29562 + 30.0 * t26175 * t114260 - 5.0 * t109926 * t7706 - 10.0 * t28602 * t29544 - 5.0 * t28602 * t29548 - 2.0 * t114270 * t2048 - 2.0 * t114296 * t2048 - 4.0 * t29538 * t7964 - 5.0 * t7343 * t114288 - 5.0 * t7343 * t114301;
    (t115348,)
}
