//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 522/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk522<F: Float>(t366: F, t4857: F, t1065: F, t905: F, t1032: F, t1647: F, t1040: F, t3147: F, t72: F, t3088: F, t3299: F, t1668: F, t3153: F, t3317: F, t1012: F, t1014: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4858 = t4857 * t366;
    let t4872 = t1065 * t905;
    let t4878 = t1647 * t1032;
    let t4879 = t4878 * t1040;
    let t4890 = t3147 * t72;
    let t4891 = t3088 * t4890;
    let t4892 = t3299 * t4891;
    let t4893 = t1668 * t3153;
    let t4899 = t3317 * t4891;
    let t4915 = t1012 * t1014;
    (t4858, t4872, t4878, t4879, t4890, t4891, t4892, t4893, t4899, t4915)
}
