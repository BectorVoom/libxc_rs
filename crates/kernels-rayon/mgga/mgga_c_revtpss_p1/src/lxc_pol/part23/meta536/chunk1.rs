//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2072/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2072(t33: f64, t6792: f64, t9350: f64, t3841: f64, t6416: f64, t1113: f64, t20256: f64, t2255: f64, t516: f64, t5557: f64, t162: f64, t21917: f64, zeta_threshold: f64) -> (f64, f64) {
    let t34 = t33 <= zeta_threshold;
    let t21918 = t9350 * t6792;
    let t21923 = t3841 * t6416;
    let t21929 = piecewise3(t34, 0.0_f64, -8.0_f64 / 27.0_f64 * t21918 * t1113 - 16.0_f64 / 9.0_f64 * t5557 * t2255 + 4.0_f64 / 9.0_f64 * t21923 * t1113 + 4.0_f64 / 3.0_f64 * t516 * t20256);
    let t21931 = (t21917 + t21929) * t162;
    (t21918, t21931)
}
