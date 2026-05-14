//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 718/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk718<F: Float>(t1022: F, t16658: F, t3413: F, t4649: F, t1952: F, t4719: F, t3450: F, t925: F, t9073: F, t446: F, t1017: F, t363: F, t2992: F, t1969: F, t2983: F, t9049: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16659 = t16658 * t1022;
    let t16661 = t4649 * t3413;
    let t16664 = t1952 * t4719;
    let t16666 = t925 * t3450;
    let t16667 = t9073 * t16666;
    let t16668 = t446 * t16667;
    let t16670 = t1017 * t363;
    let t16671 = t2992 * t16670;
    let t16672 = t1969 * t16671;
    let t16673 = t446 * t16672;
    let t16675 = t2983 * t16670;
    let t16676 = t9049 * t16675;
    (t16659, t16661, t16664, t16666, t16668, t16671, t16673, t16675, t16676)
}
