//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 347/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk347<F: Float>(t1510: F, t436: F, t619: F, t641: F, t195: F, t6: F, t134: F, t128: F, t5: F, t512: F) -> (F, F, F, F, F) {
    let t1511 = t436 * t1510;
    let t1514 = t641 * t619;
    let t1517 = t6 * t195;
    let t1518 = t1517 * t134;
    let t1521 = t1517 * t128;
    let t1524 = t5 * t512;
    (t1511, t1514, t1518, t1521, t1524)
}
