//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 721/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk721<F: Float>(t3100: F, t684: F, t1027: F, t1917: F, t1936: F, t628: F, t649: F, t3056: F, t197: F, t5479: F, t2986: F, t1018: F, t1875: F, t1877: F, t3096: F, t2990: F, t3088: F) -> (F, F, F, F, F, F, F, F) {
    let t9190 = t3100 * t684;
    let t9192 = t1027 * t1917;
    let t9194 = t628 * t1936;
    let t9195 = t9194 * t649;
    let t9197 = t628 * t3056;
    let t9198 = t9197 * t649;
    let t9200 = t197 * t5479;
    let t9201 = t2986 * t9200;
    let t9203 = t1875 * t1018;
    let t9204 = t3096 * t1877;
    let t9205 = t9203 * t9204;
    let t9207 = t3088 * t2990;
    (t9190, t9192, t9195, t9198, t9201, t9203, t9205, t9207)
}
