//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1149/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1149(t5593: f64, t81749: f64, t22690: f64, t23122: f64, t5544: f64, t841: f64, t16673: f64, t6613: f64, t28359: f64, t838: f64, t23069: f64, t5572: f64) -> (f64, f64, f64, f64, f64) {
    let t98618 = t81749 * t5593;
    let t98647 = t23122 * t22690 * t841 * t5544;
    let t98684 = t16673 * t6613;
    let t98690 = t28359 * t838;
    let t98694 = t23069 * t5572;
    (t98618, t98647, t98684, t98690, t98694)
}
