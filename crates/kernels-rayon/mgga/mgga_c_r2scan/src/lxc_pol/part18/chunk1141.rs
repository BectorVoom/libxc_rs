//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1141/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1141(t11888: f64, t8358: f64, t12595: f64, t19146: f64, t12598: f64, t6654: f64, t1070: f64, t1276: f64, t9673: f64, t11885: f64, t2928: f64, t3366: f64, t6661: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42493 = t8358 * t11888;
    let t42495 = t19146 * t12595;
    let t42497 = t6654 * t12598;
    let t42500 = t1276 * t1070 * t9673;
    let t42502 = t8358 * t11885;
    let t42505 = t6661 * t3366 * t2928;
    (t42493, t42495, t42497, t42500, t42502, t42505)
}
