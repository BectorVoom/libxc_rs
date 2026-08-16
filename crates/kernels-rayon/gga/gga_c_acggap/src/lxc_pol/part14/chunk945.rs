//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 945/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk945(t880: f64, t9062: f64, t1960: f64, t5368: f64, t310: f64, t8995: f64, t29997: f64, t7963: f64, t9029: f64, t524: f64, t9033: f64, t4241: f64, t7942: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33648 = t9062 * t880;
    let t33656 = t1960 * t5368;
    let t33662 = 0.13170898365871023197e1_f64 * t310 * t8995;
    let t33672 = 0.17347256376410398924e1_f64 * t7963 * t29997 * t9029;
    let t33673 = t9033 * t524;
    let t33681 = 0.34694512752820797848e1_f64 * t7942 * t33673 * t4241;
    (t33648, t33656, t33662, t33672, t33673, t33681)
}
