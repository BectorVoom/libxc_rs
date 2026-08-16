//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta303<F: Float>(t3014: F, t343: F, t12461: F, t3698: F, t3475: F, t460: F, t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F) -> (F, F, F, F, F, F) {
        let (t23547, t23857, t24705, t32253, t35577, t35761) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1090::<F>(t3014, t343, t12461, t3698, t3475, t460, t20, t60, t9108, t94, t102, t9174);
    (t23547, t23857, t24705, t32253, t35577, t35761)
}
