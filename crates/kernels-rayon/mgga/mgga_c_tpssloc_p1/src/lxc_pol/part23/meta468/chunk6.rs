//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1380/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1380(t10756: f64, t10771: f64, t14271: f64, t1568: f64, t1569: f64, t17499: f64, t17547: f64, t21194: f64, t21306: f64, t2861: f64, t2886: f64, t5742: f64, t5743: f64, t5758: f64, t5790: f64, t69380: f64, t76632: f64, t76663: f64, t76665: f64, t76668: f64, t76671: f64, t77001: f64, t77006: f64, t77119: f64, t77124: f64, t77127: f64, t77130: f64) -> f64 {
    let t77390 = -t76663 - t76665 - t76668 + t76671 - t77001 - t77006 + 36.0_f64 * t2886 * t5743 * t5758 - 8.0_f64 * t2861 * t1569 * t21194 + 0.61524113149298439947e4_f64 * t10756 * t17499 * t5790 + 0.3859675079686208416e3_f64 * t14271 * t21306 + 0.12865583598954028054e3_f64 * t2886 * t69380 * t1568 - 0.11579025239058625248e4_f64 * t10771 * t17547 * t5742 - 0.19751673498613801407e-1_f64 * t76632 - t77119 + t77124 - t77127 + t77130;
    t77390
}
