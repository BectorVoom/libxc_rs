//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 572/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk572(t1022: f64, t4649: f64, t1964: f64, t4417: f64, t1555: f64, t89: f64, t1017: f64, t925: f64, t1969: f64, t446: f64, t1974: f64, t356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4650 = t4649 * t1022;
    let t4652 = t1964 * t4417;
    let t4654 = t89 * t1555 * t4652;
    let t4656 = t925 * t1017;
    let t4657 = t1969 * t4656;
    let t4658 = t446 * t4657;
    let t4660 = t1974 * t4417;
    let t4662 = t89 * t356 * t4660;
    (t4650, t4652, t4654, t4656, t4657, t4658, t4660, t4662)
}
