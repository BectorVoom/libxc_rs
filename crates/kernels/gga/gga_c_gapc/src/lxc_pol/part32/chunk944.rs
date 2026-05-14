//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 944/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk944<F: Float>(t11937: F, t2639: F, t11889: F, t16408: F, t612: F, t11887: F, t7956: F, t818: F, t9066: F, t11986: F, t7939: F, t190: F, t8785: F, t1: F, t277: F, t11831: F) -> (F, F, F, F, F, F, F) {
    let t33245 = t11937 * t2639;
    let t33248 = t16408 * t612 * t11889;
    let t33252 = t11887 * t9066 * t818 * t7956;
    let t33254 = t11986 * t7939;
    let t33256 = t190 * t8785;
    let t33257 = t33256 * t1;
    let t33258 = t277 * t33257;
    let t33259 = t33258 * t11831;
    (t33245, t33248, t33252, t33254, t33257, t33258, t33259)
}
