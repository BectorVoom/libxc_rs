//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1239/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1239<F: Float>(t30160: F, t575: F, t116: F, t30004: F, t1518: F, t1936: F, t29568: F, t5891: F, t94978: F, t25823: F, t5915: F, t29694: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t105814 = t30160 * t575;
    let t105819 = t116 * t30004;
    let t105823 = t1518 * t1936;
    let t105866 = t29568 * t116;
    let t105870 = t94978 * t5891;
    let t105878 = t25823 * t5915;
    let t105933 = t29694 * t689;
    (t105814, t105819, t105823, t105866, t105870, t105878, t105933)
}
