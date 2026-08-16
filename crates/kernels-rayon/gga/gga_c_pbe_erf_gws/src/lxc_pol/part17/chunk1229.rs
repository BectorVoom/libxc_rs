//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1229/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1229(t13776: f64, t3038: f64, t3975: f64, t9504: f64, t1113: f64, t29117: f64, t50949: f64, t1114: f64, t51717: f64, t14138: f64, t3093: f64, t4386: f64) -> (f64, f64, f64, f64, f64) {
    let t52982 = t13776 * t3975 * t3038 * t9504;
    let t52986 = t13776 * t3975 * t1113 * t29117;
    let t52989 = 119.0_f64 / 3456.0_f64 * t50949;
    let t52990 = t1114 * t51717;
    let t52991 = t52990 * t14138;
    let t52992 = 7.0_f64 / 144.0_f64 * t52991;
    let t52993 = t4386 * t3093;
    (t52982, t52986, t52989, t52992, t52993)
}
