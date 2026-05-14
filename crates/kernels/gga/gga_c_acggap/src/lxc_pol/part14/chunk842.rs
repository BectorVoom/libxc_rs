//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 842/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk842<F: Float>(t1219: F, t615: F, t7911: F, t2122: F, t862: F, t865: F, t2132: F, t322: F, t7896: F, t7979: F, t2159: F, t7924: F, t1960: F, t3889: F, t2137: F, t7930: F) -> (F, F, F, F, F, F) {
    let t31965 = t615 * t7911 * t1219;
    let t31969 = t862 * t2122 * t865;
    let t31976 = t7896 * t2132 * t7979 * t322;
    let t31978 = t7924 * t2159;
    let t32001 = t1960 * t3889;
    let t32003 = t2137 * t7930;
    (t31965, t31969, t31976, t31978, t32001, t32003)
}
