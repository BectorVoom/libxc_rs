//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta293<F: Float>(t4392: F, t699: F, t13602: F, t2904: F, t4471: F, t13550: F, t13563: F, t1543: F, t2791: F, t2970: F, t4343: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t13644, t13645, t13650, t13662, t13675, t13679, t13709, t13712, t13727, t13750) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1452::<F>(t4392, t699, t13602, t2904, t4471, t13550, t13563, t1543, t2791, t2970, t4343, t973);
    (t13644, t13645, t13650, t13662, t13675, t13679, t13709, t13712, t13727, t13750)
}
