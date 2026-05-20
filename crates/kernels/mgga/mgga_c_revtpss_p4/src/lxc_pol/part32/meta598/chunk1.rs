//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1933/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933<F: Float>(t1513: F, t4287: F, t25826: F, t25823: F, t5915: F, t665: F, t21876: F, t6998: F, t28166: F, t7897: F, t5824: F, t775: F) -> (F, F, F, F, F, F) {
    let t105875 = t1513 * t4287;
    let t105876 = t25826 * t105875;
    let t105878 = t25823 * t5915;
    let t105880 = t5915 * t665;
    let t105881 = t25826 * t105880;
    let t105883 = t6998 * t21876;
    let t105892 = t7897 * t28166;
    let t105898 = t5824 * t775;
    (t105876, t105878, t105881, t105883, t105892, t105898)
}
