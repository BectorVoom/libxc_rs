//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 845/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk845<F: Float>(t2862: F, t4162: F, t882: F, t4129: F, t824: F, t319: F, t12001: F, t4159: F, t2842: F, t668: F, t2844: F, t992: F, t2881: F, t4241: F, t681: F, t89: F) -> (F, F, F, F, F) {
    let t15172 = t2862 * t882 * t4162;
    let t15175 = t4129 * t824;
    let t15177 = t2862 * t319 * t15175;
    let t15180 = t12001 * t4159;
    let t15182 = t2842 * t668;
    let t15183 = t992 * t2844;
    let t15184 = t15182 * t15183;
    let t15185 = t2881 * t15184;
    let t15190 = 2.0 / 9.0 * t89 * t681 * t4241;
    (t15172, t15177, t15180, t15185, t15190)
}
