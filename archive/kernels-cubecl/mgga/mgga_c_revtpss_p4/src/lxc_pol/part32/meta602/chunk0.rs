//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1937/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1937<F: Float>(t18521: F, t27261: F, t25222: F, t6030: F, t18423: F, t25234: F, t5993: F, t18414: F, t2661: F, t93082: F, t18418: F, t25227: F) -> (F, F, F, F, F, F) {
    let t106012 = t27261 * t18521;
    let t106014 = t25222 * t6030;
    let t106022 = t25234 * t18423;
    let t106024 = t25222 * t5993;
    let t106030 = t2661 * t93082 * t18414;
    let t106033 = t2661 * t25227 * t18418;
    (t106012, t106014, t106022, t106024, t106030, t106033)
}
