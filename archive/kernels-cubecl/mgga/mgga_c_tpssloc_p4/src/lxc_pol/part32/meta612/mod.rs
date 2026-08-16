//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2011;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta612<F: Float>(t22813: F, t6589: F, t80782: F, t23124: F, t23138: F, t6604: F, t6606: F, t22690: F, t2627: F, t10024: F, t1899: F, t2693: F, t6609: F, t213: F, t9223: F, t6593: F, t22715: F, t229: F, t805: F, t1891: F, t192: F, t80881: F, t841: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81902, t81903, t81911, t81912, t81914, t81921, t81928) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2011::<F>(t22813, t6589, t80782, t23124, t23138, t6604, t6606, t22690, t2627, t10024, t1899, t2693, t6609);
        let (t81933, t81934, t81942, t81943, t81954) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2012::<F>(t213, t6589, t9223, t6593, t22715, t229, t805, t1891, t192, t22690, t80881, t841);
    (t81902, t81903, t81911, t81912, t81914, t81921, t81928, t81933, t81934, t81942, t81943, t81954)
}
