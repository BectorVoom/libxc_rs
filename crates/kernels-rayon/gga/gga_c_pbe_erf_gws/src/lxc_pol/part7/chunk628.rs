//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 628/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk628(t422: f64, t4896: f64, t1815: f64, t639: f64, t1733: f64, t626: f64, t1809: f64, t1620: f64, t1775: f64, t583: f64, t220: f64, t2735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4897 = t4896 * t422;
    let t4898 = t1815 * t4897;
    let t4900 = 4.0_f64 / 15.0_f64 * t639 * t4898;
    let t4901 = t1733 * t626;
    let t4902 = t4901 * t422;
    let t4903 = t1809 * t4902;
    let t4905 = 8.0_f64 / 15.0_f64 * t1620 * t4903;
    let t4906 = t1775 * t583;
    let t4907 = 4.0_f64 / 15.0_f64 * t4906;
    let t4908 = t2735 * t220;
    (t4897, t4898, t4900, t4901, t4902, t4903, t4905, t4907, t4908)
}
