//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 798/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk798<F: Float>(t1413: F, t1868: F, t547: F, t807: F, t221: F, t3979: F, t3978: F, t1885: F, t3930: F, t1353: F, t4012: F, t828: F, t3826: F, t187: F, t5566: F, t1856: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5617 = t1413 * t1868;
    let t5618 = t547 * t5617;
    let t5619 = t807 * t5618;
    let t5622 = t3979 * t221 * t1868;
    let t5623 = t3978 * t5622;
    let t5625 = t3930 * t1885;
    let t5627 = t1868 * t1353;
    let t5629 = t4012 * t828 * t5627;
    let t5632 = 0.18311447306006545054e-3 * t3826;
    let t5634 = 0.19751673498613801407e-1 * t5566 * t187;
    let t5635 = t1856 * t72;
    (t5617, t5618, t5619, t5622, t5623, t5625, t5627, t5629, t5632, t5634, t5635)
}
