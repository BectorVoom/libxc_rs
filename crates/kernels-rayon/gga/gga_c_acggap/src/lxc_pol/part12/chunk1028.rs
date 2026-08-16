//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1028/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1028(t30318: f64, t532: f64, t1569: f64, t7614: f64, t1988: f64, t8838: f64, t1089: f64, t1459: f64, t33878: f64, t598: f64, t1980: f64, t33883: f64, t7458: f64) -> (f64, f64, f64, f64, f64) {
    let t34293 = t30318 * t532;
    let t34295 = t7614 * t1569;
    let t34297 = t1988 * t8838;
    let t34301 = t598 * t1089 * t1459 * t33878;
    let t34305 = t1980 * t7458 * t1459 * t33883;
    (t34293, t34295, t34297, t34301, t34305)
}
