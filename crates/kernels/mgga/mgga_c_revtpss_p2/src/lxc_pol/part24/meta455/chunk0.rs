//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1422/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1422<F: Float>(t4522: F, t874: F, t9288: F, t1573: F, t40317: F, t10867: F, t1568: F, t4503: F, t786: F, t40270: F, t4496: F, t10115: F, t1576: F) -> (F, F, F, F, F, F) {
    let t51445 = t874 * t4522 * t9288;
    let t51452 = t40317 * t1573;
    let t51498 = t10867 * t1568;
    let t51548 = t4503 * t1568;
    let t51549 = t786 * t51548;
    let t51553 = t40270 * t4496;
    let t51578 = t10115 * t1576;
    (t51445, t51452, t51498, t51549, t51553, t51578)
}
