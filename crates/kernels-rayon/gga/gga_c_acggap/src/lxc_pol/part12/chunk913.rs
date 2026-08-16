//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 913/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk913(t30318: f64, t431: f64, t1973: f64, t7780: f64, t1985: f64, t30179: f64, t1029: f64, t7614: f64, t1998: f64, t3697: f64, t1997: f64, t3243: f64, t390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30878 = t30318 * t431;
    let t30880 = t7780 * t1973;
    let t30882 = t30179 * t1985;
    let t30884 = t7614 * t1029;
    let t30886 = t1998 * t3697;
    let t30889 = t3243 * t1997 * t390;
    (t30878, t30880, t30882, t30884, t30886, t30889)
}
