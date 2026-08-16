//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1122/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1122(t2001: f64, t4878: f64, t30225: f64, t542: f64, t1588: f64, t7605: f64, t5232: f64, t2327: f64, t7610: f64, t537: f64, t1576: f64, t1581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35947 = t2001 * t4878;
    let t35949 = t30225 * t542;
    let t35951 = t7605 * t1588;
    let t35953 = t2001 * t5232;
    let t35955 = t7610 * t2327;
    let t35959 = t30225 * t537;
    let t35961 = t7605 * t1576;
    let t35963 = t7605 * t1581;
    (t35947, t35949, t35951, t35953, t35955, t35959, t35961, t35963)
}
