//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1165;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta365<F: Float>(t43776: F, t2296: F, t3241: F, t11778: F, t154: F, t22715: F, t268: F, t405: F, t39267: F, t404: F, t410: F, t407: F) -> (F, F, F, F, F, F, F) {
        let (t43777, t43791, t43809, t43819, t43820, t43880, t43889) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1165::<F>(t43776, t2296, t3241, t11778, t154, t22715, t268, t405, t39267, t404, t410, t407);
    (t43777, t43791, t43809, t43819, t43820, t43880, t43889)
}
