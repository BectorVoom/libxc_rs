//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1157/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1157<F: Float>(t105945: F, t7063: F, t30105: F, t689: F, t1032: F, t6888: F, t1426: F, t1955: F, t786: F, t6871: F, t94429: F, t22102: F, t94423: F) -> (F, F, F, F, F, F, F) {
    let t106387 = t7063 * t105945;
    let t108138 = t30105 * t689;
    let t108277 = t6888 * t1032;
    let t108278 = t108277 * t1426;
    let t108279 = t7063 * t108278;
    let t108282 = t1955 * t108277;
    let t108379 = t786 * t108278;
    let t108516 = t94429 * t6871;
    let t108524 = t94423 * t22102;
    (t106387, t108138, t108279, t108282, t108379, t108516, t108524)
}
