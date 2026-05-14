//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 906/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk906<F: Float>(t116: F, t29421: F, t1203: F, t471: F, t11239: F, t1811: F, t1828: F, t1774: F, t7642: F, t1214: F, t1769: F, t1518: F, t1936: F, t670: F, t8151: F, t84: F, t8440: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t104115 = t29421 * t116;
    let t104504 = t471 * t1203;
    let t104527 = t1811 * t11239;
    let t105236 = t1828 * t1203;
    let t105270 = t1774 * t1203;
    let t105364 = t7642 * t1811;
    let t105460 = t1769 * t1214;
    let t105823 = t1518 * t1936;
    let t111734 = t8151 * t670;
    let t119457 = t8440 * t84;
    (t104115, t104504, t104527, t105236, t105270, t105364, t105460, t105823, t111734, t119457)
}
