//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta238<F: Float>(t5723: F, t699: F, t5769: F, t942: F, t5737: F, t923: F, t2932: F, t5790: F, t10632: F, t5774: F, t2844: F, t5726: F, t2888: F, t5758: F, t10629: F, t225: F, t5849: F, t5851: F, t1040: F, t5904: F, t248: F, t3101: F, t5867: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17290, t17355, t17428, t17492, t17499, t17520) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892::<F>(t5723, t699, t5769, t942, t5737, t923, t2932, t5790, t10632, t5774, t2844, t5726);
        let (t17547, t17564, t17575, t17588, t17607, t17611) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk893::<F>(t2888, t5758, t10629, t5774, t225, t5849, t5851, t1040, t5904, t248, t3101, t5867);
    (t17290, t17355, t17428, t17492, t17499, t17520, t17547, t17564, t17575, t17588, t17607, t17611)
}
