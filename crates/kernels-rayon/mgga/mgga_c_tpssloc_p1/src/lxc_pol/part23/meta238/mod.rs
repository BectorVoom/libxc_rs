//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta238 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk893;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta238(t5723: f64, t699: f64, t5769: f64, t942: f64, t5737: f64, t923: f64, t2932: f64, t5790: f64, t10632: f64, t5774: f64, t2844: f64, t5726: f64, t2888: f64, t5758: f64, t10629: f64, t225: f64, t5849: f64, t5851: f64, t1040: f64, t5904: f64, t248: f64, t3101: f64, t5867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17290, t17355, t17428, t17492, t17499, t17520) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk892(t5723, t699, t5769, t942, t5737, t923, t2932, t5790, t10632, t5774, t2844, t5726);
        let (t17547, t17564, t17575, t17588, t17607, t17611) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk893(t2888, t5758, t10629, t5774, t225, t5849, t5851, t1040, t5904, t248, t3101, t5867);
    (t17290, t17355, t17428, t17492, t17499, t17520, t17547, t17564, t17575, t17588, t17607, t17611)
}
