//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 857/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk857(t30055: f64, t19: f64, t3220: f64, t336: f64, t151: f64, t177: f64, t3558: f64, t587: f64, t2008: f64, t980: f64, t3646: f64, t588: f64) -> (f64, f64, f64, f64, f64) {
    let t30056 = 0.15724046144802076034e-3_f64 * t30055;
    let t30058 = t3220 * t19 * t336;
    let t30077 = t151 * t587 * t3558 * t177;
    let t30078 = 0.7558530601555998074e-1_f64 * t30077;
    let t30080 = t980 * t2008 * t177;
    let t30081 = 0.60023625365297631762e-2_f64 * t30080;
    let t30083 = t3646 * t588 * t177;
    (t30056, t30058, t30078, t30081, t30083)
}
