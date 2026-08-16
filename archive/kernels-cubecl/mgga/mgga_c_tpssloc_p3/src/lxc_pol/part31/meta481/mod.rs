//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1643;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1644;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta481<F: Float>(t26396: F, t6637: F, t6888: F, t6914: F, t7737: F, t1351: F, t1834: F, t550: F, t6976: F, t1992: F, t3807: F, t5335: F, t22633: F, t5345: F, t1799: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26397, t26398, t26406, t26410, t26411, t26412, t26414) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1643::<F>(t26396, t6637, t6888, t6914, t7737, t1351, t1834, t550, t6976, t1992, t3807, t5335);
        let (t26415, t26416, t26418, t26419, t26421) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1644::<F>(t26414, t6976, t22633, t5345, t1992, t1799, t562);
    (t26397, t26398, t26406, t26410, t26411, t26412, t26414, t26415, t26416, t26418, t26419, t26421)
}
