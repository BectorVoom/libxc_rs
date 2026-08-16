//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 757/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk757<F: Float>(t637: F, t8900: F, t1510: F, t2982: F, t3084: F, t3131: F, t3707: F, t1030: F, t4979: F, t1631: F, t190: F, t1743: F) -> (F, F, F, F, F, F, F, F) {
    let t8901 = t8900 * t637;
    let t8903 = t2982 * t1510;
    let t8904 = t3084 * t8903;
    let t8906 = t3131 * t3707;
    let t8907 = t1030 * t8906;
    let t8908 = t8907 * t4979;
    let t8910 = t1631 * t190;
    let t8911 = t8910 * t3707;
    let t8912 = t1743 * t8911;
    (t8901, t8903, t8904, t8906, t8908, t8910, t8911, t8912)
}
