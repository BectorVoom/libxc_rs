//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 986/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk986<F: Float>(t11428: F, t291: F, t1461: F, t1084: F, t2763: F, t332: F, t10078: F, t6: F, t11597: F, t3415: F, t644: F, t825: F) -> (F, F, F, F, F, F, F, F) {
    let t11922 = t11428 * t291;
    let t11923 = t1461 * t11922;
    let t11924 = t1084 * t11923;
    let t11925 = t2763 * t332;
    let t11927 = t11925 * t6 * t10078;
    let t11928 = t11924 * t11927;
    let t11930 = t1084 * t11597;
    let t11931 = t11930 * t3415;
    let t11933 = t825 * t644;
    (t11923, t11924, t11925, t11927, t11928, t11930, t11931, t11933)
}
