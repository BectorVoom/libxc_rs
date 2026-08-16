//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 749/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk749(t148: f64, t4598: f64, t1326: f64, t427: f64, t40: f64, t1318: f64, t80: f64, t75: f64, t1216: f64, t455: f64) -> (f64, f64, f64, f64, f64) {
    let t4600 = 0.83762820535504401876e-1_f64 * t148 * t4598;
    let t4601 = t427 * t1326;
    let t4602 = t40 * t4601;
    let t4605 = 1.0_f64 / t1318 / t80;
    let t4606 = t75 * t4605;
    let t4607 = t1216 * t455;
    (t4600, t4602, t4605, t4606, t4607)
}
