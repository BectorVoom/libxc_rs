//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta679 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta679(t11126: f64, t4875: f64, t14858: f64, t3415: f64, t11294: f64, t4869: f64, t15044: f64, t3411: f64, t11300: f64, t1164: f64, t14841: f64, t3419: f64, t3423: f64, t51839: f64, t51844: f64, t51847: f64, t51851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51853, t51855, t51857, t51859, t51862, t51864) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2563(t11126, t4875, t14858, t3415, t11294, t4869, t15044, t3411, t11300, t1164, t14841, t3419);
        let (t51866, t51867) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2564(t14858, t3423, t51839, t51844, t51847, t51851, t51853, t51855, t51857, t51859, t51862, t51864);
    (t51853, t51855, t51857, t51859, t51862, t51864, t51866, t51867)
}
