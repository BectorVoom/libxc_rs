//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 755/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk755<F: Float>(t5091: F, t5121: F, t5130: F, t5132: F, t5134: F, t5139: F, t5141: F, t5144: F, t5148: F, t5150: F, t5154: F, t5157: F, t5159: F) -> F {
    let t5318 = t5091 + t5121 - t5130 + t5132 - t5134 - t5139 + t5141 - t5144 - t5148 + t5150 - t5154 - t5157 - t5159;
    t5318
}
