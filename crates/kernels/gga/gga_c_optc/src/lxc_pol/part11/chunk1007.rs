//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1007/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1007<F: Float>(t2839: F, t5474: F, t11927: F, t4230: F, t190: F, t2837: F, t5245: F, t5243: F, t11760: F, t1570: F, t11899: F, t5328: F, t8446: F, t1150: F, t5403: F, t7274: F) -> (F, F, F, F, F, F, F) {
    let t43834 = t5474 * t2839;
    let t43865 = t4230 * t11927;
    let t43891 = t2837 * t190 * t5245;
    let t43892 = t5243 * t43891;
    let t43906 = t1570 * t11760;
    let t44001 = t11899 * t5328;
    let t44014 = t8446 * t5328;
    let t44077 = t1150 * t7274 * t5403;
    (t43834, t43865, t43892, t43906, t44001, t44014, t44077)
}
