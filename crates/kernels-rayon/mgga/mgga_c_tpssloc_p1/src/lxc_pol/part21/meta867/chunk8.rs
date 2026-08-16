//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3172/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3172(t11709: f64, t18356: f64, t18975: f64, t3490: f64, t3540: f64, t6165: f64, t19083: f64, t3523: f64, t19026: f64, t3572: f64, t19033: f64, t11734: f64, t19095: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65660 = t11709 * t18356;
    let t65662 = t3490 * t18975;
    let t65664 = t6165 * t3540;
    let t65668 = t19083 * t3523;
    let t65670 = t19026 * t3572;
    let t65672 = t19033 * t3523;
    let t65674 = t11734 * t19095;
    (t65660, t65662, t65664, t65668, t65670, t65672, t65674)
}
