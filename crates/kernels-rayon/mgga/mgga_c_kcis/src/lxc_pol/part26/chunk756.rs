//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 756/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk756(t673: f64, t680: f64, t8698: f64, t2372: f64, t8656: f64, t2354: f64, t2698: f64, t678: f64, t2375: f64, t2366: f64, t56: f64, t649: f64, t691: f64) -> (f64, f64, f64, f64, f64) {
    let t8700 = t673 * t8698 * t680;
    let t8704 = t2372 * t8656 * t680;
    let t8708 = t2354 * t678 * t2698;
    let t8712 = t2375 * t678;
    let t8713 = t2372 * t2366 * t8712;
    let t8717 = t649 * t691 * t56;
    (t8700, t8704, t8708, t8713, t8717)
}
