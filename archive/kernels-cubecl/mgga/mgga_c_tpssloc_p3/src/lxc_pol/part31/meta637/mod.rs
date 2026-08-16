//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta637<F: Float>(t28192: F, t80727: F, t1307: F, t1377: F, t22633: F, t22635: F, t6460: F, t1842: F, t26331: F, t26337: F, t26189: F, t26193: F, t6888: F, t22892: F, t7691: F, t90544: F, t1835: F, t254: F, t28200: F, t6883: F, t90739: F, t1845: F, t5187: F, t191: F, t192: F, t19537: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97664, t97705, t97724, t97729) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904::<F>(t28192, t80727, t1307, t1377, t22633, t22635, t6460, t1842, t26331, t26337, t26189, t26193, t6888);
        let (t97732, t97740, t97750, t97766, t97789, t97804) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1905::<F>(t22892, t7691, t90544, t1835, t254, t28200, t6883, t6888, t90739, t1845, t5187, t191, t192, t19537);
    (t97664, t97705, t97724, t97729, t97732, t97740, t97750, t97766, t97789, t97804)
}
