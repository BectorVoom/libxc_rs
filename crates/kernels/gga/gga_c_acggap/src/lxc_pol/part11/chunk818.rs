//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 818/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk818<F: Float>(t1983: F, t30692: F, t3196: F, t7586: F, t1530: F, t7336: F, t1992: F, t7842: F, t30148: F, t3176: F, t7585: F, t174: F, t30423: F, t3126: F, t3157: F, t7323: F) -> (F, F, F, F, F) {
    let t30695 = t30692 * t7586 * t1983 * t3196;
    let t30698 = t1530 * t7336;
    let t30705 = t30692 * t7842 * t1992 * t3196;
    let t30709 = t7585 * t7842 * t30148 * t3176;
    let t30714 = t30423 * t7323 * t174 * t3157 * t3126;
    (t30695, t30698, t30705, t30709, t30714)
}
