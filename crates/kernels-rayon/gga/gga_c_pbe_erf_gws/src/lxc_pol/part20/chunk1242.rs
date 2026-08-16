//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1242/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1242(t53577: f64, t13972: f64, t14799: f64, t22509: f64, t4166: f64, t1176: f64, t21518: f64, t367: f64, t14602: f64, t51666: f64, t14460: f64, t4414: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53578 = 7.0_f64 / 144.0_f64 * t53577;
    let t53583 = t13972 * t14799;
    let t53584 = 7.0_f64 / 1152.0_f64 * t53583;
    let t53585 = t22509 * t4166;
    let t53592 = t1176 * t367 * t21518;
    let t53597 = t51666 * t14602;
    let t53598 = 7.0_f64 / 576.0_f64 * t53597;
    let t53610 = 7.0_f64 / 72.0_f64 * t4414 * t14460;
    (t53578, t53584, t53585, t53592, t53598, t53610)
}
