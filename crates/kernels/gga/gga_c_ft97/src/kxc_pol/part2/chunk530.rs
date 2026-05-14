//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 530/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk530<F: Float>(t3780: F, t709: F, t1701: F, t1127: F, t25: F, t3762: F, t224: F, t226: F, t2426: F, t1103: F, t172: F, t228: F, t231: F, t227: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t3781 = t3780 * t709;
    let t3782 = t1701 * t3781;
    let t3785 = t1127 * t25;
    let t3786 = t3785 * t3762;
    let t3789 = t224 * t226;
    let t3790 = t2426 * t1127;
    let t3791 = t3790 * t709;
    let t3794 = t1103 * t172;
    let t3796 = t228 * t3794 * t231;
    let t3799 = t9 * t227 * t1103;
    (t3782, t3785, t3786, t3789, t3790, t3791, t3796, t3799)
}
