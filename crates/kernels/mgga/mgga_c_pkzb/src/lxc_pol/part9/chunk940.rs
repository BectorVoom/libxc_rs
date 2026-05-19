//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 940/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk940<F: Float>(t2787: F, t5771: F, t2783: F, t683: F, t1855: F, t1084: F, t1893: F, t1856: F, t2786: F, t5776: F, t1901: F, t2782: F) -> (F, F, F, F, F, F, F, F) {
    let t7268 = F::cast_from(0.32163958997385070134e2_f64) * t5771 * t2787;
    let t7269 = t2783 * t683;
    let t7271 = F::new(4.0) * t1855 * t7269;
    let t7272 = t1084 * t1893;
    let t7274 = F::new(2.0) * t1855 * t7272;
    let t7275 = t2786 * t1856;
    let t7277 = F::cast_from(0.96491876992155210402e2_f64) * t5776 * t7275;
    let t7278 = t2782 * t1901;
    (t7268, t7269, t7271, t7272, t7274, t7275, t7277, t7278)
}
