//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1122/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1122<F: Float>(t10462: F, t1697: F, t1130: F, t4772: F, t2835: F, t4768: F, t14453: F, t291: F, t9916: F, t417: F, t9874: F, t1003: F, t1704: F) -> (F, F, F, F, F, F, F) {
    let t42972 = t1697 * t10462;
    let t43053 = t1130 * t4772;
    let t43526 = t4768 * t2835;
    let t44544 = t14453 * t291;
    let t44575 = t9916 * t291;
    let t44657 = t417 * t9874;
    let t44658 = t1704 * t1003;
    (t42972, t43053, t43526, t44544, t44575, t44657, t44658)
}
