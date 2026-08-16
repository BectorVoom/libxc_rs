//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta235 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1100;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1101;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1102;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta235<F: Float>(t3815: F, t1788: F, t588: F, t592: F, t3829: F, t3833: F, t2426: F, t2486: F, t3819: F, t3821: F, t3825: F, t3827: F, t3832: F, t5169: F, t225: F, t5262: F, t546: F, t68: F, t1365: F, t1799: F, t1307: F, t1347: F, t5187: F, t1345: F, t1348: F, t1819: F, t1821: F, t548: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5263, t5265, t5267, t5268, t5269, t5270) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1100::<F>(t3815, t1788, t588, t592, t3829, t3833, t2426, t2486, t3819, t3821, t3825, t3827, t3832, t5169);
        let (t5272, t5278, t5279, t5280, t5283, t5286) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1101::<F>(t225, t5262, t5270, t546, t68, t1365, t1799, t1307, t1347, t5187, t1345, t1348, t1819, t1821, t548);
        let t5287 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1102::<F>(t5286, t550);
    (t5263, t5265, t5267, t5268, t5269, t5272, t5278, t5279, t5280, t5283, t5286, t5287)
}
