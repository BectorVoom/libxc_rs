//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1243;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta419<F: Float>(t21127: F, t690: F, t21131: F, t699: F, t21135: F, t21139: F, t21119: F, t21697: F, t3216: F, t21238: F, t2929: F, t21334: F, t892: F, t21347: F, t300: F) -> (F, F, F, F, F, F, F, F, F) {
        let t68498 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1243::<F>(t21127, t690);
        let (t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1244::<F>(t21131, t699, t21135, t21139, t21119, t21697, t3216, t21238, t2929, t21334, t892, t21347, t300);
    (t68498, t68500, t68502, t68504, t68506, t68711, t68902, t68924, t69012)
}
