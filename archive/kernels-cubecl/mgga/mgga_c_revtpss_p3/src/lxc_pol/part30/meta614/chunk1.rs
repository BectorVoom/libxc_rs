//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2119/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2119<F: Float>(t28043: F, t4254: F, t1310: F, t28042: F, t651: F, t25851: F, t4248: F, t1518: F, t2319: F, t1937: F, t4292: F, t648: F) -> (F, F, F, F, F, F) {
    let t98474 = F::cast_from(4.0_f64) * t4254 * t28043;
    let t98477 = F::cast_from(4.0_f64) * t651 * t1310 * t28042;
    let t98483 = F::cast_from(2.0_f64) * t4248 * t25851;
    let t98484 = t2319 * t1518;
    let t98486 = F::cast_from(2.0_f64) * t98484 * t1937;
    let t98487 = t648 * t4292;
    (t98474, t98477, t98483, t98484, t98486, t98487)
}
