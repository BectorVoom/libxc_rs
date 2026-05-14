//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 747/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk747<F: Float>(t2035: F, t8764: F, t118: F, t1932: F, t2007: F, t2127: F, t2163: F, t508: F, t569: F, t8449: F, t8456: F, t8458: F, t8463: F, t8569: F, t8597: F, t8601: F, t8741: F, t8743: F, t8750: F, t8756: F, t8761: F) -> (F,) {
    let t8765 = t8764 * t2035;
    let t8766 = -t118 * t8756 - t1932 * t2163 - t2007 * t2127 - t508 * t8741 + t569 * t8761 - 2.0 * t8449 - t8456 - 2.0 * t8458 - t8463 + t8569 + t8597 - t8601 - 2.0 * t8743 - 2.0 * t8750 + t8765;
    (t8766,)
}
