//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1138/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1138<F: Float>(t3778: F, t468: F, t415: F, t3783: F, t454: F, t3787: F, t3583: F, t9447: F, t1312: F, t3575: F, t3952: F, t3733: F, t9469: F, t20: F, t394: F, t4153: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32119 = t468 * t3778;
    let t32120 = t415 * t32119;
    let t32122 = t454 * t3783;
    let t32123 = t32122 * t3787;
    let t32124 = t415 * t32123;
    let t32126 = t9447 * t3583;
    let t32127 = t1312 * t32126;
    let t32130 = t9447 * t3575;
    let t32131 = t3952 * t32130;
    let t32138 = t9469 * t3733;
    let t32139 = t415 * t32138;
    let t32142 = t4153 * t394 * t20;
    (t32119, t32120, t32123, t32124, t32126, t32127, t32130, t32131, t32138, t32139, t32142)
}
