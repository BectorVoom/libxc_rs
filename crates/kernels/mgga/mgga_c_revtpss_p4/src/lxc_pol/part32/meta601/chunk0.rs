//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1936/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1936<F: Float>(t18471: F, t25270: F, t18446: F, t18629: F, t18428: F, t27261: F, t18651: F, t18639: F, t18643: F, t92955: F, t18456: F, t6037: F, t92951: F) -> (F, F, F, F, F, F, F, F, F) {
    let t105993 = t25270 * t18471;
    let t105995 = t25270 * t18446;
    let t105997 = t25270 * t18629;
    let t105999 = t27261 * t18428;
    let t106001 = t25270 * t18651;
    let t106003 = t25270 * t18639;
    let t106006 = t92955 * t18643;
    let t106008 = t27261 * t18456;
    let t106010 = t92951 * t6037;
    (t105993, t105995, t105997, t105999, t106001, t106003, t106006, t106008, t106010)
}
