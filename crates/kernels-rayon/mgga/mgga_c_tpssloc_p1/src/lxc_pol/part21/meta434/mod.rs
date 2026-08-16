//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta434 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1970;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta434(t1714: f64, t4899: f64, t11571: f64, t11545: f64, t60: f64, t461: f64, t14726: f64, t11589: f64, t4904: f64, t3447: f64, t11588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15390, t15391, t15394, t15395) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1970(t1714, t4899, t11571, t11545, t60, t461);
        let (t15396, t15399, t15401, t15402) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1971(t14726, t15395, t11589, t4904, t3447, t11588, t461);
    (t15390, t15391, t15394, t15395, t15396, t15399, t15401, t15402)
}
