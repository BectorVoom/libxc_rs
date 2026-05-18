//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1153/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1153<F: Float>(t29682: F, t689: F, t1032: F, t6041: F, t867: F, t786: F, t18643: F, t92955: F, t6037: F, t92951: F, t25222: F, t6030: F) -> (F, F, F, F, F, F, F) {
    let t105936 = t29682 * t689;
    let t105944 = t6041 * t1032;
    let t105945 = t105944 * t867;
    let t105946 = t786 * t105945;
    let t106006 = t92955 * t18643;
    let t106010 = t92951 * t6037;
    let t106014 = t25222 * t6030;
    (t105936, t105944, t105945, t105946, t106006, t106010, t106014)
}
