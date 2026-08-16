//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1210/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1210<F: Float>(t101785: F, t101929: F, t109926: F, t114260: F, t114270: F, t114288: F, t114296: F, t114301: F, t2048: F, t26175: F, t28602: F, t29538: F, t29544: F, t29548: F, t29562: F, t30543: F, t7343: F, t7706: F, t7709: F, t7964: F) -> F {
    let t115348 = -F::cast_from(2.0_f64) * t7709 * t30543 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t101929 + F::cast_from(30.0_f64) * t101785 * t29562 + F::cast_from(30.0_f64) * t26175 * t114260 - F::cast_from(5.0_f64) * t109926 * t7706 - F::cast_from(10.0_f64) * t28602 * t29544 - F::cast_from(5.0_f64) * t28602 * t29548 - F::cast_from(2.0_f64) * t114270 * t2048 - F::cast_from(2.0_f64) * t114296 * t2048 - F::cast_from(4.0_f64) * t29538 * t7964 - F::cast_from(5.0_f64) * t7343 * t114288 - F::cast_from(5.0_f64) * t7343 * t114301;
    t115348
}
