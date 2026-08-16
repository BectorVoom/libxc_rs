//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta411 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta411<F: Float>(t13975: F, t4594: F, t4582: F, t3120: F, t3131: F, t4593: F, t10482: F, t3040: F, t3132: F, t3069: F, t4669: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13976, t13977, t13980, t13981, t13982, t13985, t13986, t13987, t13990, t13991, t13995) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1815::<F>(t13975, t4594, t4582, t3120, t3131, t4593, t10482, t3040, t3132, t3069, t4669);
    (t13976, t13977, t13980, t13981, t13982, t13985, t13986, t13987, t13990, t13991, t13995)
}
