//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1111/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1111<F: Float>(t27186: F, t98849: F, t29643: F, t686: F, t72: F, t93281: F, t93317: F, t18643: F, t92955: F, t6037: F, t92951: F, t25222: F, t6030: F, t18423: F, t25234: F, t5993: F) -> (F, F, F, F, F, F, F, F) {
    let t105962 = t98849 * t27186;
    let t105973 = t29643 * t72 * t686;
    let t105974 = t93281 * t105973;
    let t105976 = t93317 * t105973;
    let t106006 = t92955 * t18643;
    let t106010 = t92951 * t6037;
    let t106014 = t25222 * t6030;
    let t106022 = t25234 * t18423;
    let t106024 = t25222 * t5993;
    (t105962, t105974, t105976, t106006, t106010, t106014, t106022, t106024)
}
