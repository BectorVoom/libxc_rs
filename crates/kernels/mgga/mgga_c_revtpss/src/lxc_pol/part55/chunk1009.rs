//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1009/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1009<F: Float>(t596: F, t8464: F, t31746: F, t786: F, t7063: F, t31809: F, t31837: F, t10867: F, t239: F, t8478: F, t8484: F, t124: F, t800: F, t815: F, t886: F, t32474: F) -> (F, F, F, F, F, F) {
    let t120068 = t8464 * t596;
    let t120070 = t786 * t120068 * t31746;
    let t120073 = t7063 * t120068 * t31746;
    let t120082 = t31809 * t31837;
    let t120097 = t8478 * t8484 * t10867 * t239;
    let t120106 = t815 * t800 * t124 * t886;
    let t120107 = t32474 * t120106;
    (t120070, t120073, t120082, t120097, t120106, t120107)
}
