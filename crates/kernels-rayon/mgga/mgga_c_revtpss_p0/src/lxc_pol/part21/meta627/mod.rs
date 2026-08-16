//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta627 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2391;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta627(t10705: f64, t10716: f64, t10697: f64, t136: f64, t10627: f64, t221: f64, t2674: f64, t2452: f64, t9720: f64, t225: f64, t268: f64, t2665: f64, t10868: f64, t240: f64, t10871: f64, t2661: f64, t40479: f64, t10726: f64, t2723: f64, t10638: f64, t231: f64, t243: f64, t2662: f64, t10722: f64, t2656: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40681, t40683, t40686, t40688, t40689, t40690, t40691) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2390(t10705, t10716, t10697, t136, t10627, t221, t2674, t2452, t9720, t225, t268, t2665);
        let (t40693, t40696, t40700, t40705, t40707) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2391(t10868, t240, t10871, t2661, t40479, t10726, t2723, t10638, t231, t243, t2662, t10722, t2656);
    (t40681, t40683, t40686, t40688, t40689, t40690, t40691, t40693, t40696, t40700, t40705, t40707)
}
