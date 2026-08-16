//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1722/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1722(t6396: f64, t6400: f64, t1102: f64, t198: f64, t3336: f64, t336: f64, t41937: f64, t88510: f64, t88562: f64, t88564: f64, t88567: f64, t88607: f64, t88682: f64, t88986: f64, t88989: f64, t88991: f64, t88993: f64, t88995: f64, t89397: f64, t89437: f64, t89740: f64) -> f64 {
    let t89746 = t6396 * t6396;
    let t89751 = t6400 * t6400;
    let t89756 = t88510 - t88607 + t198 * t336 * (t88682 + t89397 + t89437 + t89740) * t1102 - t88562 + t88564 - t88567 - 3.0_f64 * t198 * t336 * t89746 * t3336 + t88986 - t88989 + t88991 + t88993 + t88995 - 6.0_f64 * t198 * t336 * t89751 * t41937;
    t89756
}
