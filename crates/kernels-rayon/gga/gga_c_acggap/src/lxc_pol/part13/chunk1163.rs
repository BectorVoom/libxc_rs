//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1163/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1163(t35934: f64, t31276: f64, t8544: f64, t7310: f64, t8505: f64, t2001: f64, t4894: f64, t4878: f64, t30225: f64, t542: f64, t1588: f64, t7605: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35935 = 0.21437009059034868486e-3_f64 * t35934;
    let t35936 = t31276 * t8544;
    let t35938 = t7310 * t8505;
    let t35942 = t2001 * t4894;
    let t35947 = t2001 * t4878;
    let t35949 = t30225 * t542;
    let t35951 = t7605 * t1588;
    (t35935, t35936, t35938, t35942, t35947, t35949, t35951)
}
