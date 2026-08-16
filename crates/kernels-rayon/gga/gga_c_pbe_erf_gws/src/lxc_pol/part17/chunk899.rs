//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 899/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk899(t203: f64, t7829: f64, t184: f64, t221: f64, t1406: f64, t181: f64, t997: f64, t562: f64, t577: f64, t5379: f64, t1045: f64, t1672: f64) -> (f64, f64, f64, f64, f64) {
    let t7830 = t203 * t7829;
    let t7831 = t7830 * t184;
    let t7833 = 2.0_f64 / 15.0_f64 * t7831 * t221;
    let t7834 = t1406 * t181;
    let t7835 = t7834 * t184;
    let t7837 = 4.0_f64 / 15.0_f64 * t7835 * t997;
    let t7838 = t562 * t577;
    let t7839 = t7838 * t184;
    let t7841 = 8.0_f64 / 15.0_f64 * t7839 * t997;
    let t7843 = 4.0_f64 / 15.0_f64 * t5379 * t997;
    let t7844 = t1672 * t1045;
    (t7833, t7837, t7841, t7843, t7844)
}
