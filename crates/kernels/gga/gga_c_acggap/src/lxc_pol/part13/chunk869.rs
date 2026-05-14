//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 869/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk869<F: Float>(t2159: F, t7924: F, t310: F, t7970: F, t16548: F, t7932: F, t7942: F, t2131: F, t2147: F, t463: F, t7979: F, t1960: F, t3889: F, t2137: F, t7930: F, t322: F) -> (F, F, F, F, F, F, F) {
    let t31978 = t7924 * t2159;
    let t31984 = t310 * t7970;
    let t31991 = t7942 * t7932 * t16548;
    let t31999 = t2131 * t2147 * t7979 * t463;
    let t32001 = t1960 * t3889;
    let t32003 = t2137 * t7930;
    let t32004 = t7932 * t322;
    (t31978, t31984, t31991, t31999, t32001, t32003, t32004)
}
