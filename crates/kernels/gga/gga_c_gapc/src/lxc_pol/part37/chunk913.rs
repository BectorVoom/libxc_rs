//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 913/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk913<F: Float>(t10078: F, t11925: F, t6: F, t11924: F, t1084: F, t11597: F, t3415: F, t644: F, t825: F, t311: F, t3273: F, t11499: F, t325: F, t2626: F, t3374: F) -> (F, F, F, F, F, F, F, F) {
    let t11927 = t11925 * t6 * t10078;
    let t11928 = t11924 * t11927;
    let t11930 = t1084 * t11597;
    let t11931 = t11930 * t3415;
    let t11933 = t825 * t644;
    let t11935 = t311 * t11933 * t3273;
    let t11937 = t325 * t11499;
    let t11938 = t2626 * t3374;
    (t11927, t11928, t11930, t11931, t11933, t11935, t11937, t11938)
}
