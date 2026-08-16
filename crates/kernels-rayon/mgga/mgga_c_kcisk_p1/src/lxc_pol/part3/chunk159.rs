//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 159/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk159(t311: f64, t312: f64, t579: f64, t571: f64, t574: f64, t577: f64, t573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t581 = t311 * t312 * t579;
    let t583 = 0.379785e1_f64 * t574 + 0.8969e0_f64 * t571 + 0.204775e0_f64 * t577 + 0.123235e0_f64 * t581;
    let t586 = 1.0_f64 + 0.16081824322151104822e2_f64 / t583;
    let t587 = f64::ln(t586);
    let t589 = 0.62182e-1_f64 * t573 * t587;
    let t591 = 1.0_f64 + 0.278125e-1_f64 * t571;
    (t581, t583, t586, t587, t589, t591)
}
