//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 234/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk234<F: Float>(t316: F, t880: F, t243: F, t75: F, t288: F, t98: F, t100: F, t229: F, t277: F, t224: F, t244: F, t272: F, t687: F, t791: F) -> (F, F, F, F, F, F, F, F) {
    let t882 = F::new(0.65854491829355115987e0) * t316 * t880;
    let t883 = t243 * t75;
    let t884 = t883 * t288;
    let t886 = F::new(1.0) / t98;
    let t893 = F::new(1.0) / t100;
    let t904 = t229 * t277;
    let t905 = F::new(8.0) * t904;
    let t906 = t224 * t244;
    let t912 = t791 * t687 * t272;
    (t882, t883, t884, t886, t893, t905, t906, t912)
}
