//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta386 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta386(t12344: f64, t5234: f64, t1831: f64, t40059: f64, t12282: f64, t12290: f64, t12384: f64, t1827: f64, t40123: f64, t1788: f64, t9212: f64, t9214: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1190(t12344, t5234, t1831, t40059, t12282, t12290, t12384, t1827, t40123, t1788, t9212, t9214);
    (t53880, t53901, t53945, t54020, t54042, t54151, t54312, t54314)
}
