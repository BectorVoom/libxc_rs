//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1027/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1027<F: Float>(t18643: F, t92955: F, t6037: F, t92951: F, t25222: F, t6030: F, t18423: F, t25234: F, t5993: F, t18414: F, t2661: F, t93082: F, t18418: F, t25227: F, t18402: F, t18409: F) -> (F, F, F, F, F, F, F, F, F) {
    let t106006 = t92955 * t18643;
    let t106010 = t92951 * t6037;
    let t106014 = t25222 * t6030;
    let t106022 = t25234 * t18423;
    let t106024 = t25222 * t5993;
    let t106030 = t2661 * t93082 * t18414;
    let t106033 = t2661 * t25227 * t18418;
    let t106037 = t25234 * t18402;
    let t106040 = t2661 * t25227 * t18409;
    (t106006, t106010, t106014, t106022, t106024, t106030, t106033, t106037, t106040)
}
