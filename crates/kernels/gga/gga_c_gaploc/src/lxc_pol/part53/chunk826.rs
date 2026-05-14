//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 826/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk826<F: Float>(t13851: F, t2013: F, t40986: F, t40989: F, t13883: F, t1991: F, t590: F, t13870: F, t1890: F, t1966: F, t12240: F, t2679: F, t9800: F, t3720: F, t5241: F, t9805: F) -> (F, F, F, F, F, F, F) {
    let t47151 = t2013 * t13851;
    let t47155 = 0.38342925953920749677e0 * t40986;
    let t47157 = 0.72851559312449424385e1 * t40989;
    let t47160 = 0.51123901271894332902e0 * t1991 * t13883 * t590;
    let t47164 = 0.51123901271894332902e0 * t1966 * t1890 * t13870 * t590;
    let t47166 = t9800 * t12240 * t2679;
    let t47168 = t5241 * t3720;
    let t47170 = t9805 * t47168 * t2679;
    (t47151, t47155, t47157, t47160, t47164, t47166, t47170)
}
