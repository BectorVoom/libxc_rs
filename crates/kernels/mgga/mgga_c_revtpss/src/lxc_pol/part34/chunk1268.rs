//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1268/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1268<F: Float>(t30110: F, t531: F, t1913: F, t7956: F, t30197: F, t571: F, t2045: F, t6936: F, t1921: F, t7939: F, t2037: F, t6951: F) -> (F, F, F, F, F, F) {
    let t109173 = t531 * t30110;
    let t109339 = t1913 * t7956;
    let t109345 = t571 * t30197;
    let t109348 = t6936 * t2045;
    let t109349 = t7939 * t1921;
    let t109351 = t2037 * t6951;
    (t109173, t109339, t109345, t109348, t109349, t109351)
}
