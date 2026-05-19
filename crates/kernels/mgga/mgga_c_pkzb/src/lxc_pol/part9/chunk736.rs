//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 736/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk736<F: Float>(t124: F, t5119: F, t1535: F, t4867: F, t4870: F, t4873: F, t4876: F, t4879: F, t4881: F, t4884: F, t4887: F, t5077: F, t5079: F, t5081: F, t5082: F, t5087: F, t5091: F, t568: F) -> (F, F) {
    let t5121 = F::cast_from(0.19751673498613801407e-1_f64) * t5119 * t124;
    let t5122 = -F::new(9.0) * t1535 * t5082 * t568 + t4867 + t4870 + t4873 - t4876 - t4879 + t4881 + t4884 + t4887 + t5077 + t5079 - t5081 - t5087 + t5091 + t5121;
    (t5121, t5122)
}
