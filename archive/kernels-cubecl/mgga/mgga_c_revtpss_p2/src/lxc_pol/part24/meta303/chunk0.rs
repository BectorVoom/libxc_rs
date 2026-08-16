//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1088/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1088<F: Float>(t1256: F, t6595: F, t6598: F, t17183: F, t5330: F, t1811: F, t5219: F, t1284: F, t6564: F, t6688: F, t73: F, t3766: F) -> (F, F, F, F, F, F, F) {
    let t21285 = t6595 * t1256;
    let t21287 = t6598 * t1256;
    let t21306 = t17183 * t5330;
    let t21394 = t5219 * t1811;
    let t21439 = t6564 * t1284;
    let t21442 = t6688 * t73;
    let t21451 = t3766 * t1811;
    (t21285, t21287, t21306, t21394, t21439, t21442, t21451)
}
