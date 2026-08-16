//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta688 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta688<F: Float>(t13084: F, t13258: F, t13353: F, t9638: F, t41466: F, t820: F, t13176: F, t2642: F, t10024: F, t1500: F, t13293: F, t9573: F) -> (F, F, F, F, F, F) {
        let (t47027, t47037, t47039, t47044, t47047, t47049) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2503::<F>(t13084, t13258, t13353, t9638, t41466, t820, t13176, t2642, t10024, t1500, t13293, t9573);
    (t47027, t47037, t47039, t47044, t47047, t47049)
}
