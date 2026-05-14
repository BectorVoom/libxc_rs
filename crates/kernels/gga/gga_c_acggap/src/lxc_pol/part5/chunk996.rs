//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 996/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk996<F: Float>(t1160: F, t4162: F, t6482: F, t3077: F, t6535: F, t4210: F, t6461: F, t1539: F, t1907: F, t406: F, t377: F, t6510: F, t1922: F, t980: F, t381: F, t6454: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t19862 = t1160 * t6482 * t4162;
    let t19864 = t3077 * t6535;
    let t19870 = t1160 * t6461 * t4210;
    let t19874 = t1160 * t1907 * t406 * t1539;
    let t19880 = t377 * t6510;
    let t19882 = t980 * t1922;
    let t19885 = t381 * t6454 * t879;
    (t19862, t19864, t19870, t19874, t19880, t19882, t19885)
}
