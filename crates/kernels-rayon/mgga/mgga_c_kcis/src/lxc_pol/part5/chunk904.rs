//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 904/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk904(t8604: f64, t97: f64, t128: f64, t66: f64, t15: f64, t736: f64, t32: f64, t5: f64, t742: f64, t103: f64, t2357: f64, t119: f64, t681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8605 = t97 * t8604;
    let t8611 = t66 * t128;
    let t8618 = t736 * t15;
    let t8630 = 0.34451131037037037036e-2_f64 * t5 * t742 * t32;
    let t8631 = t103 * t2357;
    let t8634 = t119 * t681;
    (t8605, t8611, t8618, t8630, t8631, t8634)
}
