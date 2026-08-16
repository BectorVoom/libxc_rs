//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2308/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2308<F: Float>(t1260: F, t17307: F, t17183: F, t5330: F, t1774: F, t3736: F, t1811: F, t3766: F, t460: F, t3781: F, t3302: F, t471: F) -> (F, F, F, F, F, F, F, F) {
    let t21275 = t17307 * t1260;
    let t21306 = t17183 * t5330;
    let t21389 = t3736 * t1774;
    let t21451 = t3766 * t1811;
    let t21452 = t460 * t21451;
    let t21455 = t3781 * t1811;
    let t21456 = t460 * t21455;
    let t21471 = t3302 * t471;
    (t21275, t21306, t21389, t21451, t21452, t21455, t21456, t21471)
}
