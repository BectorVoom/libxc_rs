//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1225/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1225(t1113: f64, t13776: f64, t29117: f64, t3975: f64, t1114: f64, t51717: f64, t14138: f64, t3093: f64, t4386: f64, t13792: f64, t3089: f64, t1162: f64, t14682: f64, t2158: f64, t3989: f64) -> (f64, f64, f64, f64, f64) {
    let t52986 = t13776 * t3975 * t1113 * t29117;
    let t52990 = t1114 * t51717;
    let t52991 = t52990 * t14138;
    let t52993 = t4386 * t3093;
    let t52994 = t13792 * t52993;
    let t52996 = t4386 * t3089;
    let t52997 = t13792 * t52996;
    let t53009 = t3989 * t14682 * t1162 * t2158;
    (t52986, t52991, t52994, t52997, t53009)
}
