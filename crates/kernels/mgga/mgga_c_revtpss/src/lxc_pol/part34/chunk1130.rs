//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1130/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1130<F: Float>(t3252: F, t65: F, t3204: F, t7131: F, t4817: F, t7132: F, t7810: F, t994: F, t1976: F, t4746: F, t1035: F, t1982: F, t27418: F) -> (F, F, F, F, F, F, F) {
    let t27531 = t65 * t3252;
    let t27536 = t3204 * t7131;
    let t27539 = t7132 * t4817;
    let t27550 = t994 * t7810;
    let t27568 = t4746 * t1976;
    let t27604 = t1035 * t7810;
    let t27609 = t1982 * t27418;
    (t27531, t27536, t27539, t27550, t27568, t27604, t27609)
}
