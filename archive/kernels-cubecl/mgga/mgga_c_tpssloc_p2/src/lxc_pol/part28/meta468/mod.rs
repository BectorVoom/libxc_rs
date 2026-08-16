//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta468 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta468<F: Float>(t25154: F, t25155: F, t253: F, t254: F, t1484: F, t857: F, t865: F, t23270: F, t22986: F, t23204: F, t7488: F, t6562: F) -> (F, F, F, F, F, F, F, F) {
        let (t25156, t25168, t25191, t25192, t25193, t25194, t25205, t25206) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1677::<F>(t25154, t25155, t253, t254, t1484, t857, t865, t23270, t22986, t23204, t7488, t6562);
    (t25156, t25168, t25191, t25192, t25193, t25194, t25205, t25206)
}
