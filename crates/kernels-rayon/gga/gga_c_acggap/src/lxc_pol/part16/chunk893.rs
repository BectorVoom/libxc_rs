//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 893/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk893(t3378: f64, t7432: f64, t2074: f64, t12726: f64, t2067: f64, t2070: f64, t3360: f64, t8462: f64, t7560: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30792 = t3378 * t7432;
    let t30793 = t30792 * t2074;
    let t30797 = t12726 * t2067;
    let t30798 = t30797 * t2070;
    let t30806 = t3360 * t8462;
    let t30811 = t3360 * t7560;
    (t30792, t30793, t30797, t30798, t30806, t30811)
}
