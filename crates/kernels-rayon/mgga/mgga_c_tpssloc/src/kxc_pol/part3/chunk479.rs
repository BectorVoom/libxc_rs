//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 479/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk479(t1687: f64, t449: f64, t1150: f64, t1153: f64, t1655: f64, t1662: f64, t1665: f64, t1668: f64) -> (f64, f64) {
    let t1688 = t1687 * t449;
    let t1694 = 0.258925e1_f64 * t1662 - t1150 + 0.301925e0_f64 * t1655 + 0.16504875e0_f64 * t1665 - t1153 + 0.82785e-1_f64 * t1668;
    (t1688, t1694)
}
