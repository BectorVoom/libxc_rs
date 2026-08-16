//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta371 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1817;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta371(t4392: f64, t699: f64, t13611: f64, t908: f64, t136: f64, t13602: f64, t13598: f64, t13613: f64, t13630: f64, t13632: f64, t13635: f64, t13638: f64, t13640: f64, t13642: f64, t10300: f64, t10556: f64, t10558: f64, t10560: f64, t10562: f64, t10675: f64, t10676: f64, t13530: f64, t13534: f64, t13539: f64, t13544: f64, t13548: f64, t13551: f64, t13552: f64, t13557: f64, t13561: f64, t13563: f64, t13592: f64, t13616: f64, t13624: f64, t13626: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t13644, t13645, t13646, t13647, t13650, t13652) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1817(t4392, t699, t13611, t908, t136, t13602, t13598, t13613, t13630, t13632, t13635, t13638, t13640, t13642);
        let t13654 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1818(t10300, t10556, t10558, t10560, t10562, t10675, t10676, t13530, t13534, t13539, t13544, t13548, t13551, t13552, t13557, t13561, t13563, t13592, t13616, t13624, t13626, t13652);
    (t13644, t13645, t13646, t13647, t13650, t13654)
}
