//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta466<F: Float>(t1509: F, t236: F, t23110: F, t232: F, t23109: F, t1898: F, t4162: F, t249: F, t1496: F, t23069: F, t4257: F, t6621: F) -> (F, F, F, F, F, F, F) {
        let (t25130, t25132, t25133, t25135, t25136, t25140, t25142) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1675::<F>(t1509, t236, t23110, t232, t23109, t1898, t4162, t249, t1496, t23069, t4257, t6621);
    (t25130, t25132, t25133, t25135, t25136, t25140, t25142)
}
