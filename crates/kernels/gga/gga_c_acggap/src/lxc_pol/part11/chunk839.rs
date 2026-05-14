//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 839/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk839<F: Float>(t2085: F, t4210: F, t13299: F, t31115: F, t1988: F, t7681: F, t1095: F, t1980: F, t30058: F, t3116: F, t7310: F, t7389: F, t7753: F, t7799: F, t7380: F, t7381: F, t839: F) -> (F, F, F, F, F, F, F) {
    let t31116 = t2085 * t4210;
    let t31118 = t31115 * t13299 * t31116;
    let t31120 = t1988 * t7681;
    let t31124 = t1980 * t30058 * t1095 * t3116;
    let t31126 = t7310 * t7389;
    let t31128 = t7799 * t7753;
    let t31131 = t7380 * t7381 * t839;
    (t31116, t31118, t31120, t31124, t31126, t31128, t31131)
}
