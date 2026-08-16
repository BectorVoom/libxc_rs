//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1255/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1255(t1032: f64, t8085: f64, t1955: f64, t120988: f64, t120995: f64, t122309: f64, t122312: f64, t122315: f64, t122319: f64, t125599: f64, t125603: f64, t28003: f64, t32250: f64, t32690: f64, t5774: f64, t7298: f64, t8706: f64, t8708: f64) -> (f64, f64, f64) {
    let t128617 = t8085 * t1032;
    let t128618 = t1955 * t128617;
    let t128625 = -t120988 + 0.17347256376410398924e1_f64 * t32690 * t28003 + 0.7437465841810202164e-3_f64 * t125599 - t122309 + 0.28559868832551176308e-1_f64 * t122312 - 0.50779446784275991476e-1_f64 * t122315 - 0.14874931683620404328e-2_f64 * t125603 + t122319 + 0.17347256376410398924e1_f64 * t128618 * t7298 - 0.17135921299530705785e1_f64 * t8706 * t32250 * t8708 * t5774 + t120995;
    (t128617, t128618, t128625)
}
