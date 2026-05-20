//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1200/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1200<F: Float>(t1774: F, t8197: F, t7637: F, t2148: F, t6695: F, t1287: F, t6622: F, t7660: F, t26907: F, t3769: F, t6628: F, t1769: F) -> (F, F, F, F, F) {
    let t30866 = t8197 * t1774;
    let t30867 = t7637 * t30866;
    let t30870 = t2148 * t6695;
    let t30874 = t7660 * t6622 * t1287;
    let t30878 = t26907 * t6628 * t3769;
    let t30881 = t1769 * t1769;
    (t30867, t30870, t30874, t30878, t30881)
}
