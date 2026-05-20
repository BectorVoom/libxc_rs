//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1181/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1181<F: Float>(t23474: F, t930: F, t141: F, t11142: F, t23470: F, t128: F) -> (F, F, F, F) {
    let t23475 = t930 * t23474;
    let t23476 = t141 * t23475;
    let t23478 = t11142 * t23470;
    let t23479 = t128 * t23478;
    (t23475, t23476, t23478, t23479)
}
