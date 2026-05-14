//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1025/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1025<F: Float>(t3664: F, t8903: F, t3691: F, t8728: F, t1023: F, t1386: F, t3669: F, t11578: F, t1952: F, t619: F, t1030: F, t11428: F, t11591: F, t1461: F, t505: F, t11439: F, t129: F, t19670: F) -> (F, F, F, F, F, F) {
    let t34617 = t3664 * t8903;
    let t34619 = t3691 * t8728;
    let t34622 = t1386 * t3669 * t1023;
    let t34625 = t11578 * t1952 * t619;
    let t34630 = t1030 * t1461 * t11428 * t505 * t11591;
    let t34633 = t19670 * t129 * t11439;
    (t34617, t34619, t34622, t34625, t34630, t34633)
}
