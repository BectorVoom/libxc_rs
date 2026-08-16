//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1045/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1045(t2030: f64, t4586: f64, t7815: f64, t7447: f64, t8800: f64, t30219: f64, t8661: f64, t7575: f64, t7576: f64, t8480: f64, t30543: f64, t8446: f64) -> (f64, f64, f64, f64, f64) {
    let t34604 = t2030 * t7815 * t4586;
    let t34609 = t7447 * t8800;
    let t34611 = t30219 * t8661;
    let t34614 = t7575 * t8480 * t7576;
    let t34616 = t30543 * t8446;
    (t34604, t34609, t34611, t34614, t34616)
}
