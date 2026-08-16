//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 639/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk639(t172: f64, t3706: f64, t1017: f64, t513: f64, t398: f64, t1456: f64, t997: f64, t2606: f64, t2610: f64, t2613: f64, t2616: f64, t2622: f64, t2625: f64, t2629: f64, t2634: f64, t2642: f64, t2826: f64, t3994: f64, t3995: f64, t4029: f64, t4031: f64, t4032: f64, t4036: f64) -> (f64, f64, f64, f64, f64) {
    let t5011 = t172 * t3706;
    let t5012 = t513 * t1017;
    let t5014 = t398 * t5011 * t5012;
    let t5017 = t997 * t1456;
    let t5019 = -t2606 + t2610 + t3994 + t2613 + t2616 + t3995 - t2622 - t2625 + t4029 + t2629 - t2634 + t4031 - t4032 - t2642 + t4036 + t2826;
    (t5011, t5012, t5014, t5017, t5019)
}
