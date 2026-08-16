//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 577/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk577(t1529: f64, t310: f64, t1633: f64, t157: f64, t864: f64, t1629: f64, t3088: f64, t1642: f64, t3378: f64, t1539: f64, t4166: f64, t1160: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4180 = t310 * t1529;
    let t4182 = 0.26341796731742046394e1_f64 * t4180 * t1633;
    let t4183 = t157 * t864;
    let t4184 = t1629 * t4183;
    let t4185 = t3088 * t4184;
    let t4188 = 0.13170898365871023197e1_f64 * t3378 * t1642;
    let t4189 = t4166 * t1539;
    let t4191 = 0.13170898365871023197e1_f64 * t1160 * t4189;
    (t4180, t4182, t4183, t4185, t4188, t4191)
}
