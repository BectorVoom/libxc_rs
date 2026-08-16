//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1960/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1960(t12261: f64, t12297: f64, t16706: f64, t16876: f64, t17115: f64, t17117: f64, t20268: f64, t20274: f64, t20276: f64, t20278: f64, t20280: f64, t20322: f64, t20338: f64, t20341: f64, t20344: f64, t20347: f64, t20350: f64, t20353: f64, t20357: f64, t20359: f64, t20362: f64, t20380: f64) -> f64 {
    let t20382 = 0.91983333333333333333e-1_f64 * t12261 - t17115 - t17117 - 0.27595e-1_f64 * t20268 + 0.26837777777777777779e0_f64 * t16706 + 0.18396666666666666667e0_f64 * t16876 + 0.82785e-1_f64 * t20274 + 0.18396666666666666667e-1_f64 * t20276 - 0.11038e0_f64 * t20278 - 0.5519e-1_f64 * t20280 + t20322 + 0.258925e1_f64 * t20338 + 0.16557e0_f64 * t20341 - 0.5519e-1_f64 * t20344 - 0.16557e0_f64 * t20347 + 0.33114e0_f64 * t20350 + 0.49671e0_f64 * t20353 + 0.13418888888888888889e0_f64 * t12297 + 0.19419375e1_f64 * t20357 - 0.258925e1_f64 * t20359 - 0.1294625e1_f64 * t20362 + t20380;
    t20382
}
