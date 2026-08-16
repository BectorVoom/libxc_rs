//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta730 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2585;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta730<F: Float>(t3447: F, t44584: F, t4904: F, t44510: F, t14753: F, t15402: F, t14744: F, t1174: F, t135: F, t15359: F, t11589: F, t15293: F, t15382: F, t44525: F, t11588: F, t4928: F, t3451: F, t15357: F, t3448: F, t14740: F, t15419: F, t11584: F, t15338: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51980, t51988, t51991, t51995, t52013, t52019) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2585::<F>(t3447, t44584, t4904, t44510, t14753, t15402, t14744, t1174, t135, t15359, t11589, t15293);
        let (t52022, t52036, t52038, t52040, t52050, t52053) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2586::<F>(t15382, t3447, t44525, t11588, t4928, t3451, t15357, t3448, t14740, t15419, t11584, t15338);
    (t51980, t51988, t51991, t51995, t52013, t52019, t52022, t52036, t52038, t52040, t52050, t52053)
}
