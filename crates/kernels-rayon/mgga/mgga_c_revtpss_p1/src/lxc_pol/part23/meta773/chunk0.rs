//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2577/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2577(t57726: f64, t12248: f64, t1732: f64, t3433: f64, t56176: f64, t56183: f64, t56228: f64, t12429: f64, t1744: f64, t12469: f64, t1737: f64, t3362: f64, t462: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t57727 = t57726 / 324.0_f64;
    let t57818 = t12248 * t1732;
    let t57854 = t3433 * t1732;
    let t57872 = 0.15829629629629629629e-1_f64 * t56176;
    let t57874 = 0.47488888888888888888e-1_f64 * t56183;
    let t57889 = 0.23744444444444444444e-1_f64 * t56228;
    let t57944 = t12429 * t1744;
    let t58005 = t1737 * t12469;
    let t58027 = t462 * t3362;
    (t57727, t57818, t57854, t57872, t57874, t57889, t57944, t58005, t58027)
}
