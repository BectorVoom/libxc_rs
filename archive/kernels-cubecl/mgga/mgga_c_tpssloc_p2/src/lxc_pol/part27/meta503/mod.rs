//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1896;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta503<F: Float>(t25241: F, t6646: F, t1888: F, t23110: F, t7524: F, t23185: F, t234: F, t6604: F, t1484: F, t252: F) -> (F, F, F, F, F, F) {
        let (t25242, t25243, t25245, t25246, t25248) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1896::<F>(t25241, t6646, t1888, t23110, t7524, t23185, t234, t6604);
        let t25249 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1897::<F>(t1484, t252);
    (t25242, t25243, t25245, t25246, t25248, t25249)
}
