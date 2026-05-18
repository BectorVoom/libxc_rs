//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1104/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1104<F: Float>(t11603: F, t5250: F, t2839: F, t5474: F, t11927: F, t4230: F, t190: F, t2837: F, t5245: F, t5243: F, t11760: F, t1570: F) -> (F, F, F, F, F) {
    let t43809 = t5250 * t11603;
    let t43834 = t5474 * t2839;
    let t43865 = t4230 * t11927;
    let t43891 = t2837 * t190 * t5245;
    let t43892 = t5243 * t43891;
    let t43906 = t1570 * t11760;
    (t43809, t43834, t43865, t43892, t43906)
}
