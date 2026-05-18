//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1092/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1092<F: Float>(t27383: F, t34097: F, t1468: F, t1962: F, t1544: F, t1583: F, t1940: F, t198: F, t207: F, t2403: F, t26590: F, t28460: F, t32491: F, t34079: F, t34090: F, t7432: F, t7782: F, t8657: F, t892: F) -> (F, F, F) {
    let t34098 = t27383 * t34097;
    let t34100 = t1468 * t1962;
    let t34126 = t198 * t207 * t34079 * t892 + F::new(3.0) * t1544 * t2403 * t8657 - t1583 * t1940 * t32491 - t1940 * t1962 * t28460 + F::new(2.0) * t1940 * t26590 * t34097 - t1940 * t7432 * t7782 - F::new(3.0) * t2403 * t34090 * t7432;
    (t34098, t34100, t34126)
}
