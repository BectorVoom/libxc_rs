//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1180/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1180<F: Float>(t11543: F, t8751: F, t11425: F, t3085: F, t3664: F, t8903: F, t3691: F, t8728: F, t1023: F, t1386: F, t3669: F, t11578: F, t1952: F, t619: F) -> (F, F, F, F, F, F) {
    let t34613 = t11543 * t8751;
    let t34615 = t11425 * t3085;
    let t34617 = t3664 * t8903;
    let t34619 = t3691 * t8728;
    let t34622 = t1386 * t3669 * t1023;
    let t34625 = t11578 * t1952 * t619;
    (t34613, t34615, t34617, t34619, t34622, t34625)
}
