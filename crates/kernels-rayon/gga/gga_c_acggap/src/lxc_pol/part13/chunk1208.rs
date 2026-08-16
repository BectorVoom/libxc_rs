//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1208/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1208(t694: f64, t8379: f64, t560: f64, t922: f64, t105: f64, t1953: f64, t2163: f64, t24605: f64, t2541: f64, t29938: f64, t29950: f64, t33393: f64, t33397: f64, t33403: f64, t33409: f64, t33412: f64, t33441: f64, t33478: f64, t33517: f64, t33558: f64, t33588: f64, t33628: f64, t33666: f64, t33711: f64, t33755: f64, t33775: f64, t33810: f64, t36425: f64, t36463: f64, t36489: f64, t36528: f64, t36566: f64, t469: f64, t5399: f64, t567: f64, t7301: f64, t8372: f64, t8387: f64, t9096: f64, t9097: f64) -> f64 {
    let t36575 = 6.0_f64 * t694 * t8379;
    let t36577 = t560 * t922;
    let t36585 = -2.0_f64 * t567 * t2163 * t5399 + 3.0_f64 * t567 * t1953 * t33393 - 12.0_f64 * t8372 * t2541 * t33397 - t33403 + 4.0_f64 * t9096 * t9097 * t24605 + t33409 + t33412 + t567 * t105 * (t33441 + t33478 + t33517 + t33558 + t33588 + t33628 + t33666 + t33711 + t33755 + t33775 + t33810 + t36425 + t36463 + t36489 + t36528 + t36566) * t469 - t36575 - 6.0_f64 * t29938 - 6.0_f64 * t8372 * t2541 * t36577 + 6.0_f64 * t29950 + 3.0_f64 * t567 * t8387 * t7301;
    t36585
}
