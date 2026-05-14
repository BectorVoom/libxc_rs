//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 719/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk719<F: Float>(t16676: F, t446: F, t1882: F, t4657: F, t358: F, t4714: F, t363: F, t1969: F, t4668: F, t9073: F, t15756: F, t569: F, t3281: F, t4462: F, t558: F, t15768: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16677 = t446 * t16676;
    let t16679 = t1882 * t4657;
    let t16681 = t4714 * t358;
    let t16682 = t16681 * t363;
    let t16683 = t1969 * t16682;
    let t16684 = t446 * t16683;
    let t16686 = t4668 * t358;
    let t16687 = t16686 * t363;
    let t16688 = t9073 * t16687;
    let t16689 = t446 * t16688;
    let t16691 = t569 * t15756;
    let t16692 = t3281 * t16691;
    let t16694 = t4462 * t558;
    let t16695 = t1969 * t16694;
    let t16696 = t446 * t16695;
    let t16698 = t569 * t15768;
    (t16677, t16679, t16682, t16684, t16687, t16689, t16692, t16694, t16696, t16698)
}
