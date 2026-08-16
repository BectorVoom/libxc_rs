//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1224/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1224(t14116: f64, t3973: f64, t1178: f64, t904: f64, t14688: f64, t2397: f64, t13972: f64, t14726: f64, t13808: f64, t14588: f64, t50949: f64, t1114: f64, t51717: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52906 = t3973 * t14116;
    let t52926 = t904 * t1178;
    let t52930 = t14688 * t2397;
    let t52961 = t13972 * t14726;
    let t52968 = t13808 * t14588;
    let t52989 = 119.0_f64 / 3456.0_f64 * t50949;
    let t52990 = t1114 * t51717;
    (t52906, t52926, t52930, t52961, t52968, t52989, t52990)
}
