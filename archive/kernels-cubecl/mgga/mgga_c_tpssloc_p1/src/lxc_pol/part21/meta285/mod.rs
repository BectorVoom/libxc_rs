//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1578;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta285<F: Float>(t2955: F, t969: F, t2967: F, t964: F, t340: F, t63: F, t344: F, t221: F, t339: F, t2960: F, t2974: F, t135: F, t3016: F, t973: F, t1036: F, t3078: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F, t370: F, t3158: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10331, t10333, t10335, t10337, t10339, t10342, t10352) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1578::<F>(t2955, t969, t2967, t964, t340, t63, t344, t221, t339, t2960, t2974, t135, t3016);
        let (t10353, t10370, t10372, t10375, t10377, t10381) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1579::<F>(t10352, t973, t1036, t3078, t1032, t3082, t2393, t374, t376, t370, t3158, t964);
    (t10331, t10333, t10335, t10337, t10339, t10342, t10353, t10370, t10372, t10375, t10377, t10381)
}
