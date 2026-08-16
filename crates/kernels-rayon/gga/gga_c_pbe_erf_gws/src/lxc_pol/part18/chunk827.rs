//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 827/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk827(t2561: f64, t7669: f64, t587: f64, t197: f64, t2620: f64, t1660: f64, t331: f64, t1802: f64, t1885: f64, t2566: f64, t5129: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7670 = t7669 * t2561;
    let t7672 = 16.0_f64 / 81.0_f64 * t587 * t7670;
    let t7694 = t2620 * t197;
    let t7698 = t331 * t1660;
    let t7699 = t7698 * t197;
    let t7703 = t1885 * t1802;
    let t7713 = t5129 * t2566;
    let t7715 = 16.0_f64 / 135.0_f64 * t587 * t7713;
    let t7720 = t2620 * t597;
    (t7672, t7694, t7699, t7703, t7715, t7720)
}
