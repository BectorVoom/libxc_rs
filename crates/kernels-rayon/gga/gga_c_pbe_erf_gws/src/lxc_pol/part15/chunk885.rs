//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 885/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk885(t587: f64, t7666: f64, t197: f64, t5283: f64, t2561: f64, t1000: f64, t1866: f64, t1827: f64, t1821: f64, t7350: f64, t2559: f64, t7326: f64) -> (f64, f64, f64, f64, f64) {
    let t7668 = 32.0_f64 / 135.0_f64 * t587 * t7666;
    let t7669 = t5283 * t197;
    let t7670 = t7669 * t2561;
    let t7672 = 16.0_f64 / 81.0_f64 * t587 * t7670;
    let t7673 = t1000 * t1866;
    let t7674 = t1827 * t7673;
    let t7676 = 4.0_f64 / 45.0_f64 * t587 * t7674;
    let t7677 = t1821 * t7350;
    let t7679 = 8.0_f64 / 45.0_f64 * t587 * t7677;
    let t7680 = t2559 * t7326;
    (t7668, t7672, t7676, t7679, t7680)
}
