//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 868/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk868(t313: f64, t8637: f64, t723: f64, t8528: f64, t1445: f64, t1710: f64, t2949: f64, t3031: f64, t4614: f64, t2950: f64, t4673: f64, t1035: f64, t2066: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8638 = t313 * t8637;
    let t8645 = t8528 * t723;
    let t8646 = t1445 * t8645;
    let t8649 = t2949 * t1710;
    let t8650 = t1445 * t8649;
    let t8655 = t4614 * t3031;
    let t8658 = t4673 * t2950;
    let t8663 = t2066 * t1035;
    (t8638, t8646, t8650, t8655, t8658, t8663)
}
