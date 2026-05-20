//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1589/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1589<F: Float>(t20112: F, t380: F, t1043: F, t1089: F, t6343: F, t1668: F, t4930: F, t16449: F, t1651: F, t4772: F, t5004: F, t20089: F) -> (F, F, F, F, F, F) {
    let t20113 = t380 * t20112;
    let t20119 = t6343 * t1043 * t1089;
    let t20123 = t4930 * t1668 * t1089;
    let t20128 = t16449 * t1651;
    let t20133 = t5004 * t4772;
    let t20136 = t20089 * t1089;
    (t20113, t20119, t20123, t20128, t20133, t20136)
}
