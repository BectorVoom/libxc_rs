//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 829/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk829(t1250: f64, t5728: f64, t1253: f64, t1705: f64, t935: f64, t1771: f64, t5570: f64) -> (f64, f64, f64, f64) {
    let t5729 = t5728 * t1250;
    let t5736 = t1705 * t1253;
    let t5737 = t5736 * t935;
    let t5739 = t1771 * t5570;
    (t5729, t5736, t5737, t5739)
}
