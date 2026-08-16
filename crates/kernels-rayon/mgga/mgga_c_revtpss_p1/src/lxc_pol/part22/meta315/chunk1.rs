//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1758/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1758(t10555: f64, t2611: f64, t2398: f64, t2615: f64, t2609: f64, t717: f64, t162: f64, t9544: f64) -> (f64, f64, f64, f64) {
    let t10556 = t2611 * t10555;
    let t10561 = t2398 * t2615;
    let t10563 = t717 * t2609;
    let t10565 = t162 * t9544;
    (t10556, t10561, t10563, t10565)
}
