//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2835/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2835(t141: f64, t2908: f64, t51905: f64, t15183: f64, t698: f64, t15172: f64, t2439: f64, t4625: f64, t4622: f64, t15186: f64, t51890: f64, t51892: f64, t51894: f64, t51896: f64, t51899: f64, t51902: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51907 = t141 * t2908 * t51905;
    let t51909 = t698 * t15183;
    let t51911 = t698 * t15172;
    let t51913 = t2439 * t4625;
    let t51914 = 0.5519e0_f64 * t51913;
    let t51915 = t2439 * t4622;
    let t51917 = t698 * t15186;
    let t51919 = -0.3883875e1_f64 * t51890 - 0.1294625e1_f64 * t51892 + 0.247573125e0_f64 * t51894 + 0.82524375e-1_f64 * t51896 - 0.485484375e1_f64 * t51899 + 0.6189328125e-1_f64 * t51902 - 0.82785e-1_f64 * t51907 - 0.66228e0_f64 * t51909 + 0.11038e0_f64 * t51911 + t51914 - 0.91983333333333333334e-1_f64 * t51915 - 0.33114e0_f64 * t51917;
    (t51907, t51909, t51911, t51913, t51915, t51917, t51919)
}
