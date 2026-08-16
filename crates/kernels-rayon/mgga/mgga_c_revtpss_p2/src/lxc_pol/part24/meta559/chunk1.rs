//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1679/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1679(t2924: f64, t6110: f64, t6141: f64, t41908: f64, t63453: f64, t63459: f64, t63464: f64, t77499: f64, t77559: f64, t77561: f64, t88085: f64, t88089: f64, t88093: f64, t88097: f64) -> (f64, f64) {
    let t88510 = 36.0_f64 * t2924 * t6110 * t6141;
    let t88524 = 0.4566222222222222222e-1_f64 * t77559 - 0.13698666666666666667e0_f64 * t77561 + 0.25367901234567901233e-1_f64 * t77499 - 0.3044148148148148148e-1_f64 * t63453 + 0.9132444444444444444e-1_f64 * t63459 + t41908 + 0.41096e0_f64 * t88085 - 0.61644e0_f64 * t88089 + 0.10274e0_f64 * t88093 + 0.13698666666666666667e0_f64 * t88097 - 0.45662222222222222221e-1_f64 * t63464;
    (t88510, t88524)
}
