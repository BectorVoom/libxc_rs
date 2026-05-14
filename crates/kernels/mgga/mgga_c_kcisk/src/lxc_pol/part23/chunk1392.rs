//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1392/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1392<F: Float>(t19024: F, t415: F, t9469: F, t2212: F, t3783: F, t3787: F, t33557: F, t3791: F, t32026: F, t33451: F, t32066: F, t1333: F, t33558: F, t1406: F, t5981: F, t1413: F, t5867: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114700 = t415 * t9469 * t19024;
    let t114702 = t2212 * t3783;
    let t114704 = t415 * t114702 * t3787;
    let t114707 = t415 * t33557 * t3791;
    let t114712 = 0.26805555555555555556e-2 * t32026 * t33451;
    let t114714 = 0.26805555555555555556e-2 * t32066 * t33451;
    let t114715 = t1333 * t33558;
    let t114716 = 0.33163888888888888888e-2 * t114715;
    let t114718 = t415 * t1406 * t5981;
    let t114720 = t5867 * t1413;
    (t114700, t114704, t114707, t114712, t114714, t114715, t114716, t114718, t114720)
}
