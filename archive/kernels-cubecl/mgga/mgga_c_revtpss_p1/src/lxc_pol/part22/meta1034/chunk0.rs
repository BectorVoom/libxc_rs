//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3618/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3618<F: Float>(t20273: F, t698: F, t1145: F, t141: F, t68391: F, t3417: F, t68280: F, t68285: F, t1139: F, t68463: F, t2439: F, t6467: F) -> (F, F, F, F, F, F) {
    let t68567 = t698 * t20273;
    let t68570 = t141 * t1145 * t68391;
    let t68573 = t141 * t3417 * t68280;
    let t68576 = t141 * t3417 * t68285;
    let t68578 = t1139 * t68463;
    let t68583 = t2439 * t6467;
    (t68567, t68570, t68573, t68576, t68578, t68583)
}
