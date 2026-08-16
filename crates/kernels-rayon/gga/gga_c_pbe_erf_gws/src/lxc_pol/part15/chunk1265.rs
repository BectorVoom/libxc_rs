//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1265/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1265(t53583: f64, t22509: f64, t4166: f64, t1176: f64, t21518: f64, t367: f64, t3974: f64, t3990: f64, t8939: f64, t14602: f64, t51666: f64, t3959: f64, t9704: f64) -> (f64, f64, f64, f64, f64) {
    let t53584 = 7.0_f64 / 1152.0_f64 * t53583;
    let t53585 = t22509 * t4166;
    let t53592 = t1176 * t367 * t21518;
    let t53595 = t53592 * t3990 * t3974 * t8939;
    let t53597 = t51666 * t14602;
    let t53598 = 7.0_f64 / 576.0_f64 * t53597;
    let t53599 = t3959 * t9704;
    (t53584, t53585, t53595, t53598, t53599)
}
