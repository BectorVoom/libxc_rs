//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 473/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk473<F: Float>(t1399: F, t1437: F, t1419: F, t546: F, t1431: F, t1436: F, t213: F, t820: F) -> F {
    let t1438 = t1437 * t1399;
    let t1441 = t546 * t1419;
    let t1444 = -t1431 + t1436 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1438 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t1441;
    t1444
}
