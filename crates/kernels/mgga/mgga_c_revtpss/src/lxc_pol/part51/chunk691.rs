//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 691/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk691<F: Float>(t532: F, t8598: F, t4147: F, t2014: F, t118: F, t1932: F, t2007: F, t508: F, t569: F, t8447: F, t8449: F, t8456: F, t8458: F, t8463: F, t8557: F, t8565: F, t8569: F, t8597: F) -> (F, F, F) {
    let t8599 = t532 * t8598;
    let t8600 = t8599 * t4147;
    let t8601 = t2014 * t8600;
    let t8602 = -t118 * t8557 - 2.0 * t1932 * t2007 - t508 * t8447 + t569 * t8565 - 4.0 * t8449 - t8456 - 4.0 * t8458 - t8463 + 2.0 * t8569 + t8597 - t8601;
    (t8599, t8600, t8602)
}
