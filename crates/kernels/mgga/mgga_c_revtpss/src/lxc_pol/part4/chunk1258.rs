//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1258/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1258<F: Float>(t3568: F, t5486: F, t1287: F, t1794: F, t3727: F, t1770: F, t3766: F, t3759: F, t5245: F, t5457: F, t5351: F, t13126: F, t487: F, t460: F, t12050: F, t3601: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t17917 = t5486 * t3568;
    let t17921 = t3727 * t1794 * t1287;
    let t17934 = t1770 * t3766;
    let t17941 = t3759 * t5245;
    let t17944 = t5457 * t3568;
    let t17945 = t5351 * t17944;
    let t17948 = t13126 * t487;
    let t17949 = t460 * t17948;
    let t17951 = t12050 * t3601 * t471;
    (t17917, t17921, t17934, t17941, t17945, t17949, t17951)
}
