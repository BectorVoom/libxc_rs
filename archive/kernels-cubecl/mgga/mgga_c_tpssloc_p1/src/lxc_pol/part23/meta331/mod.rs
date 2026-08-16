//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta331 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta331<F: Float>(t20: F, t60: F, t9108: F, t94: F, t102: F, t9174: F, t16: F, t2: F, t591: F, t21: F, t9: F, t587: F, t598: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t32253, t35577, t35761, t39030, t39031, t39032, t39033, t39034, t39035) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1099::<F>(t20, t60, t9108, t94, t102, t9174, t16, t2, t591, t21, t9, t587, t598);
    (t32253, t35577, t35761, t39030, t39031, t39032, t39033, t39034, t39035)
}
