//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta435 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1972;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1973;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta435(t15402: f64, t4729: f64, t3447: f64, t14736: f64, t4900: f64, t14740: f64, t14731: f64, t11575: f64, t4904: f64, t134: f64, t3439: f64, t461: f64, t4724: f64, t11514: f64, t11556: f64, t11558: f64, t11561: f64, t15391: f64, t15396: f64, t15401: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15403, t15405, t15406, t15409, t15412, t15415, t15418, t15419) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1972(t15402, t4729, t3447, t14736, t4900, t14740, t14731, t11575, t4904, t134, t3439, t461);
        let (t15420, t15422, t15423) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1973(t15419, t4724, t3447, t11514, t11556, t11558, t11561, t15391, t15396, t15401, t15405, t15406, t15409, t15412, t15415);
    (t15403, t15405, t15406, t15409, t15412, t15415, t15418, t15419, t15420, t15422, t15423)
}
