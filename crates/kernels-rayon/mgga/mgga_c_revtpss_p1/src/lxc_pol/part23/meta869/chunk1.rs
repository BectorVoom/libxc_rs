//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2767/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2767(t22212: f64, t2626: f64, t1320: f64, t22195: f64, t221: f64, t22253: f64, t4018: f64, t4019: f64, t125: f64, t21969: f64, t1399: f64, t6883: f64, t9816: f64, t9818: f64) -> (f64, f64, f64, f64, f64) {
    let t74130 = t22212 * t2626;
    let t74132 = t1320 * t22195;
    let t74174 = t4018 * t4019 * t221 * t22253;
    let t74177 = t125 * t21969;
    let t74184 = t9816 * t9818 * t6883 * t1399;
    (t74130, t74132, t74174, t74177, t74184)
}
