//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 963/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk963(t43970: f64, t664: f64, t1356: f64, t2010: f64, t72162: f64, t8465: f64, t2415: f64, t72171: f64, t7349: f64, t75016: f64, t75020: f64, t75022: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77416 = t43970 * t664;
    let t77418 = 0.39914139006212695214e-1_f64 * t1356 * t77416;
    let t77420 = t2010 * t8465 * t72162;
    let t77421 = 0.36021158228745895953e-3_f64 * t77420;
    let t77423 = t7349 * t2415 * t72171;
    let t77424 = 0.5124043883133942371e-4_f64 * t77423;
    let t77425 = 0.2553875993597870364e-4_f64 * t75016;
    let t77426 = 0.2553875993597870364e-4_f64 * t75020;
    let t77427 = 0.3830813990396805546e-4_f64 * t75022;
    (t77416, t77418, t77421, t77424, t77425, t77426, t77427)
}
