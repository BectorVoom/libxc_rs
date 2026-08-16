//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta550 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta550<F: Float>(t3788: F, t6388: F, t6936: F, t1339: F, t6420: F, t6417: F, t6945: F, t1827: F, t26233: F, t6415: F, t22839: F, t6371: F, t1998: F, t236: F, t6330: F, t22845: F, t6347: F, t6926: F, t6375: F, t6916: F, t26246: F, t26268: F, t27012: F, t27019: F, t27022: F, t27027: F) -> (F, F, F, F, F, F) {
        let (t28057, t28058, t28060, t28061, t28063, t28065, t28067, t28068, t28070) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1904::<F>(t3788, t6388, t6936, t1339, t6420, t6417, t6945, t1827, t26233, t6415, t22839, t6371);
        let (t28073, t28077, t28083) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1905::<F>(t1998, t236, t6330, t22845, t6347, t6926, t6375, t6916, t26246, t26268, t27012, t27019, t27022, t27027, t28058, t28061, t28063, t28065, t28068, t28070);
    (t28057, t28060, t28067, t28073, t28077, t28083)
}
