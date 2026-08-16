//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1923/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1923<F: Float>(t1565: F, t93066: F, t25222: F, t4345: F, t4349: F, t93072: F, t14910: F, t25270: F, t14678: F, t14673: F, t92955: F, t14688: F) -> (F, F, F, F, F, F, F) {
    let t99009 = t93066 * t1565;
    let t99011 = t25222 * t4345;
    let t99013 = t93072 * t4349;
    let t99015 = t25270 * t14910;
    let t99017 = t25270 * t14678;
    let t99019 = t92955 * t14673;
    let t99021 = t92955 * t14688;
    (t99009, t99011, t99013, t99015, t99017, t99019, t99021)
}
