//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 860/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk860(t1980: f64, t30058: f64, t30059: f64, t1998: f64, t3732: f64, t151: f64, t177: f64, t3558: f64, t587: f64, t2008: f64, t980: f64, t3646: f64, t588: f64) -> (f64, f64, f64, f64, f64) {
    let t30061 = t1980 * t30058 * t30059;
    let t30073 = t1998 * t3732;
    let t30077 = t151 * t587 * t3558 * t177;
    let t30080 = t980 * t2008 * t177;
    let t30083 = t3646 * t588 * t177;
    (t30061, t30073, t30077, t30080, t30083)
}
