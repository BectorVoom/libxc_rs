//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta261<F: Float>(t107: F, t2585: F, t2281: F, t667: F, t2333: F, t626: F, t2359: F, t655: F, t93: F, t94: F, t101: F, t102: F) -> (F, F, F, F, F, F, F, F) {
        let (t9358, t9359, t9361, t9363, t9364, t9365, t9384, t9397) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1503::<F>(t107, t2585, t2281, t667, t2333, t626, t2359, t655, t93, t94, t101, t102);
    (t9358, t9359, t9361, t9363, t9364, t9365, t9384, t9397)
}
