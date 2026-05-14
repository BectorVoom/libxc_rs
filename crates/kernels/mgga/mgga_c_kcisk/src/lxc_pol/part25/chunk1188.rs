//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1188/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1188<F: Float>(t9724: F, t9999: F, t10005: F, t9736: F, t9720: F, t654: F, t7573: F, t20: F, t2801: F, t10000: F, t2807: F, t33163: F, t33165: F, t33168: F, t33196: F, t34400: F, t34406: F, t34429: F, t34435: F, t9728: F, t9743: F) -> (F, F, F, F, F, F) {
    let t34444 = t9724 * t9999;
    let t34449 = t10005 * t9736;
    let t34452 = t9720 * t9999;
    let t34455 = t7573 * t654;
    let t34456 = t34455 * t20;
    let t34457 = t2801 * t34456;
    let t34460 = -0.20104166666666666667e-2 * t33196 * t34429 - 0.17361111111111111111e-2 * t34435 * t9743 - 0.20104166666666666667e-2 * t33196 * t34400 - 0.60312500000000000001e-2 * t33196 * t34406 + 0.52083333333333333333e-2 * t10000 * t9728 + 0.20104166666666666667e-2 * t34444 * t9728 + 0.67013888888888888888e-3 * t33163 - 0.17361111111111111111e-2 * t33165 + 0.46296296296296296297e-2 * t34449 - 0.5787037037037037037e-3 * t33168 - 0.52083333333333333333e-2 * t34452 * t2807 - 0.52083333333333333333e-2 * t34457 * t2807;
    (t34444, t34452, t34455, t34456, t34457, t34460)
}
