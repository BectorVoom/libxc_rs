//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1630;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1631;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1632;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta397(t15403: f64, t3447: f64, t14736: f64, t4900: f64, t14740: f64, t14731: f64, t11575: f64, t4904: f64, t134: f64, t3439: f64, t461: f64, t4724: f64, t11514: f64, t11556: f64, t11558: f64, t11561: f64, t15391: f64, t15396: f64, t15401: f64, t15292: f64, t15330: f64, t15386: f64, t225: f64, t3507: f64, t475: f64, t6739: f64, t1755: f64, t11546: f64, t14726: f64, t15026: f64, t3032: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15405, t15406, t15409, t15412, t15415, t15418, t15420) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1630(t15403, t3447, t14736, t4900, t14740, t14731, t11575, t4904, t134, t3439, t461, t4724);
        let t15423 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1631(t15420, t3447, t11514, t11556, t11558, t11561, t15391, t15396, t15401, t15405, t15406, t15409, t15412, t15415);
        let (t15425, t15426, t15429, t15430, t15434, t15437) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1632(t15292, t15330, t15386, t15423, t225, t3507, t475, t6739, t1755, t11546, t14726, t15026, t3032);
    (t15418, t15425, t15426, t15429, t15430, t15434, t15437)
}
