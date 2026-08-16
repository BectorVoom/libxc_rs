//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 882/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk882(t617: f64, t631: f64, t184: f64, t1024: f64, t2724: f64, t633: f64, t5355: f64, t1648: f64, t2632: f64, t2784: f64, t597: f64, t562: f64) -> (f64, f64, f64, f64, f64) {
    let t7631 = t617 * t631;
    let t7632 = t7631 * t184;
    let t7634 = 8.0_f64 / 15.0_f64 * t7632 * t1024;
    let t7636 = 4.0_f64 / 15.0_f64 * t633 * t2724;
    let t7637 = 8.0_f64 / 135.0_f64 * t5355;
    let t7639 = 8.0_f64 / 15.0_f64 * t1648 * t2632;
    let t7640 = t597 * t2784;
    let t7641 = t7640 * t562;
    (t7634, t7636, t7637, t7639, t7641)
}
