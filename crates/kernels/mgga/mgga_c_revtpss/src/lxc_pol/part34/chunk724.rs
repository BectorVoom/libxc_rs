//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 724/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk724<F: Float>(t532: F, t7933: F, t1450: F, t2014: F, t2034: F, t5542: F, t118: F, t1502: F, t1519: F, t1843: F, t1911: F, t1932: F, t2007: F, t2011: F, t508: F, t569: F, t651: F, t6985: F, t7725: F, t7731: F, t7734: F, t7737: F, t7744: F, t7746: F, t7883: F, t7894: F, t7899: F, t7903: F) -> (F, F, F, F) {
    let t7934 = t532 * t7933;
    let t7935 = t7934 * t1450;
    let t7936 = t2014 * t7935;
    let t7937 = t2034 * t5542;
    let t7938 = t2014 * t7937;
    let t7939 = -t118 * t7883 - t1502 * t2007 - 2.0 * t1519 * t6985 - t1843 * t1932 + t1911 * t2011 - t508 * t7725 + t569 * t7894 - 2.0 * t651 * t7746 - t7731 - t7734 - t7737 - t7744 + t7899 + t7903 + t7936 - t7938;
    (t7934, t7935, t7937, t7939)
}
